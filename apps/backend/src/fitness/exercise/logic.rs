use anyhow::{anyhow, Result};
use async_graphql::{Enum, InputObject, SimpleObject};
use chrono::Utc;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sea_orm::{
    prelude::DateTimeUtc, ActiveModelTrait, ActiveValue, ColumnTrait, DatabaseConnection,
    EntityTrait, FromJsonQueryResult, QueryFilter,
};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    entities::{
        prelude::{Exercise, UserToExercise, Workout},
        user_to_exercise, workout,
    },
    models::fitness::{
        TotalMeasurement, UserToExerciseExtraInformation, UserToExerciseHistoryExtraInformation,
    },
};

#[skip_serializing_none]
#[derive(
    Clone,
    Debug,
    Deserialize,
    Serialize,
    FromJsonQueryResult,
    Eq,
    PartialEq,
    SimpleObject,
    InputObject,
)]
#[graphql(input_name = "SetStatisticInput")]
pub struct SetStatistic {
    pub duration: Option<Decimal>,
    pub distance: Option<Decimal>,
    pub reps: Option<Decimal>,
    pub weight: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, Enum, Copy)]
pub enum SetLot {
    Normal,
    WarmUp,
    Drop,
    Failure,
}

#[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, Enum, Copy)]
pub enum WorkoutSetPersonalBest {
    Weight,
    OneRm,
    Volume,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, SimpleObject,
)]
pub struct WorkoutSetRecord {
    pub statistic: SetStatistic,
    pub lot: SetLot,
    pub personal_bests: Vec<WorkoutSetPersonalBest>,
}

fn get_best_set(records: &[WorkoutSetRecord]) -> Option<&WorkoutSetRecord> {
    records.iter().max_by_key(|record| {
        record.statistic.duration.unwrap_or(dec!(0))
            + record.statistic.distance.unwrap_or(dec!(0))
            + record.statistic.reps.unwrap_or(dec!(0))
            + record.statistic.weight.unwrap_or(dec!(0))
    })
}

#[derive(
    Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, SimpleObject,
)]
pub struct ProcessedExercise {
    pub exercise_name: String,
    pub exercise_id: i32,
    pub sets: Vec<WorkoutSetRecord>,
    pub notes: Vec<String>,
    pub rest_time: Option<u16>,
    pub total: TotalMeasurement,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, SimpleObject,
)]
pub struct WorkoutInformation {
    /// Each grouped superset of exercises will be in a vector. They will contain
    /// the `exercise.idx`.
    pub supersets: Vec<Vec<u16>>,
    pub exercises: Vec<ProcessedExercise>,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, SimpleObject,
)]
pub struct WorkoutSummaryExercise {
    pub num_sets: usize,
    pub name: String,
    pub best_set: WorkoutSetRecord,
}

#[derive(
    Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq, SimpleObject,
)]
pub struct WorkoutSummary {
    pub total: TotalMeasurement,
    pub exercises: Vec<WorkoutSummaryExercise>,
}

#[derive(Clone, Debug, Deserialize, Serialize, InputObject)]
pub struct UserWorkoutSetRecord {
    pub statistic: SetStatistic,
    pub lot: SetLot,
}

#[derive(Clone, Debug, Deserialize, Serialize, InputObject)]
pub struct UserExerciseInput {
    pub exercise_id: i32,
    pub sets: Vec<UserWorkoutSetRecord>,
    pub notes: Vec<String>,
    pub rest_time: Option<u16>,
}

#[derive(Clone, Debug, Deserialize, Serialize, InputObject)]
pub struct UserWorkoutInput {
    pub name: Option<String>,
    pub comment: Option<String>,
    pub start_time: DateTimeUtc,
    pub end_time: DateTimeUtc,
    pub exercises: Vec<UserExerciseInput>,
    pub supersets: Vec<Vec<u16>>,
}

impl UserWorkoutInput {
    /// Create a workout in the database and also update user and exercise associations.
    pub async fn calculate_and_commit(
        self,
        user_id: i32,
        db: &DatabaseConnection,
        id: String,
    ) -> Result<String> {
        let mut exercises = vec![];
        let mut workout_totals = vec![];
        for (idx, ex) in self.exercises.into_iter().enumerate() {
            let db_ex = Exercise::find_by_id(ex.exercise_id)
                .one(db)
                .await?
                .ok_or_else(|| anyhow!("No exercise found!"))?;
            let mut sets = vec![];
            let mut total = TotalMeasurement::default();
            let association = UserToExercise::find()
                .filter(user_to_exercise::Column::UserId.eq(user_id))
                .filter(user_to_exercise::Column::ExerciseId.eq(ex.exercise_id))
                .one(db)
                .await
                .ok()
                .flatten();
            let history_item = UserToExerciseHistoryExtraInformation {
                workout_id: id.clone(),
                idx,
            };
            let (association, all_sets) = match association {
                None => {
                    let user_to_ex = user_to_exercise::ActiveModel {
                        user_id: ActiveValue::Set(user_id),
                        exercise_id: ActiveValue::Set(ex.exercise_id),
                        num_times_performed: ActiveValue::Set(1),
                        last_updated_on: ActiveValue::Set(Utc::now()),
                        extra_information: ActiveValue::Set(UserToExerciseExtraInformation {
                            history: vec![history_item],
                            lifetime_stats: TotalMeasurement::default(),
                        }),
                    };
                    (user_to_ex.insert(db).await.unwrap(), vec![])
                }
                Some(e) => {
                    let previous_workouts = e.extra_information.history.clone();
                    let previous_db_workouts = Workout::find()
                        .filter(
                            workout::Column::Id
                                .is_in(previous_workouts.iter().map(|e| e.workout_id.clone())),
                        )
                        .all(db)
                        .await?;
                    let mut all_sets = vec![];
                    for wkt in previous_db_workouts {
                        let wiq = previous_workouts
                            .iter()
                            .find(|w| w.workout_id == wkt.id)
                            .unwrap();
                        all_sets.extend(wkt.information.exercises[wiq.idx].sets.clone());
                    }
                    let performed = e.num_times_performed;
                    let mut extra_info = e.extra_information.clone();
                    extra_info.history.push(history_item);
                    let mut up: user_to_exercise::ActiveModel = e.into();
                    up.num_times_performed = ActiveValue::Set(performed + 1);
                    up.extra_information = ActiveValue::Set(extra_info);
                    up.last_updated_on = ActiveValue::Set(Utc::now());
                    (up.update(db).await?, all_sets)
                }
            };
            dbg!(&all_sets);
            for set in ex.sets {
                // FIXME: Correct calculations
                let mut personal_bests = vec![];
                if let Some(r) = set.statistic.reps {
                    total.reps += r;
                    if let Some(w) = set.statistic.weight {
                        total.weight += w * r;
                    }
                }
                if let Some(d) = set.statistic.duration {
                    total.duration += d;
                }
                if let Some(d) = set.statistic.distance {
                    total.distance += d;
                }
                total.personal_bests_achieved = personal_bests.len();
                sets.push(WorkoutSetRecord {
                    statistic: set.statistic,
                    lot: set.lot,
                    personal_bests,
                });
            }
            workout_totals.push(total.clone());
            let mut association_totals = association.extra_information.clone();
            let mut association: user_to_exercise::ActiveModel = association.into();
            association_totals.lifetime_stats += total.clone();
            association.extra_information = ActiveValue::Set(association_totals);
            association.update(db).await?;
            exercises.push(ProcessedExercise {
                exercise_id: ex.exercise_id,
                exercise_name: db_ex.name,
                sets,
                notes: ex.notes,
                rest_time: ex.rest_time,
                total,
            });
        }
        let summary_total = workout_totals.into_iter().sum();
        let model = workout::Model {
            id,
            start_time: self.start_time,
            end_time: self.end_time,
            user_id,
            name: self.name,
            comment: self.comment,
            // TODO: Mark as true when calculation code is complete.
            processed: false,
            summary: WorkoutSummary {
                total: summary_total,
                exercises: exercises
                    .iter()
                    .map(|e| WorkoutSummaryExercise {
                        num_sets: e.sets.len(),
                        name: e.exercise_name.clone(),
                        best_set: get_best_set(&e.sets).unwrap().clone(),
                    })
                    .collect(),
            },
            information: WorkoutInformation {
                supersets: self.supersets,
                exercises,
            },
        };
        let insert: workout::ActiveModel = model.into();
        let data = insert.insert(db).await?;
        Ok(data.id)
    }
}
