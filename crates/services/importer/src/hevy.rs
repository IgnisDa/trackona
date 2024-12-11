use std::{collections::HashMap, fs, sync::Arc};

use async_graphql::Result;
use chrono::{Duration, NaiveDateTime};
use common_utils::ryot_log;
use csv::ReaderBuilder;
use database_models::{exercise, prelude::Exercise};
use dependent_models::{ImportCompletedItem, ImportResult};
use dependent_utils::generate_exercise_id;
use enums::{ExerciseLot, ExerciseSource};
use fitness_models::{
    SetLot, UserExerciseInput, UserWorkoutInput, UserWorkoutSetRecord, WorkoutSetStatistic,
};
use importer_models::{ImportFailStep, ImportFailedItem};
use indexmap::IndexMap;
use itertools::Itertools;
use media_models::DeployGenericCsvImportInput;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use supporting_service::SupportingService;

use super::utils;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "PascalCase")]
struct Entry {
    #[serde(alias = "Workout #")]
    workout_number: String,
    date: String,
    #[serde(alias = "Workout Name")]
    workout_name: String,
    #[serde(alias = "Duration (sec)", alias = "Duration")]
    workout_duration: String,
    #[serde(alias = "Exercise Name")]
    exercise_name: String,
    #[serde(alias = "Set Order")]
    set_order: String,
    #[serde(alias = "Weight (kg)")]
    weight: Option<Decimal>,
    reps: Option<Decimal>,
    #[serde(alias = "Distance (m)")]
    distance: Option<Decimal>,
    seconds: Option<Decimal>,
    notes: Option<String>,
    #[serde(alias = "Workout Notes")]
    workout_notes: Option<String>,
}

pub async fn import(
    input: DeployGenericCsvImportInput,
    ss: &Arc<SupportingService>,
    user_id: &str,
) -> Result<ImportResult> {
    let mut completed = vec![];
    let mut failed = vec![];
    let file_string = fs::read_to_string(&input.csv_path)?;
    // DEV: Delimiter is `;` on android and `,` on iOS, so we determine it by reading the first line
    let data = file_string.clone();
    let first_line = data.lines().next().unwrap();
    let delimiter = if first_line.contains(';') {
        b';'
    } else if first_line.contains(',') {
        b','
    } else {
        return Err("Could not determine delimiter".into());
    };

    let mut unique_exercises: HashMap<String, exercise::Model> = HashMap::new();
    let entries_reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(file_string.as_bytes())
        .deserialize::<Entry>()
        .map(|r| r.unwrap())
        .collect_vec();

    let mut workouts_to_entries = IndexMap::new();
    for entry in entries_reader.clone() {
        workouts_to_entries
            .entry(entry.workout_number.clone())
            .or_insert(vec![])
            .push(entry);
    }

    let mut exercises_to_workouts = IndexMap::new();

    for (workout_number, entries) in workouts_to_entries {
        let mut exercises = IndexMap::new();
        for entry in entries {
            exercises
                .entry(entry.exercise_name.clone())
                .or_insert(vec![])
                .push(entry);
        }
        exercises_to_workouts.insert(workout_number, exercises);
    }

    for (_workout_number, workout) in exercises_to_workouts {
        let first_exercise = workout.first().unwrap().1.first().unwrap();
        let ndt = NaiveDateTime::parse_from_str(&first_exercise.date, "%Y-%m-%d %H:%M:%S")
            .expect("Failed to parse input string");
        let ndt = utils::get_date_time_with_offset(ndt, &ss.timezone);
        let workout_duration =
            Duration::try_seconds(first_exercise.workout_duration.parse().unwrap()).unwrap();
        let mut collected_exercises = vec![];
        for (exercise_name, exercises) in workout.clone() {
            let mut collected_sets = vec![];
            let mut notes = vec![];
            let valid_ex = exercises.iter().find(|e| e.set_order != "Note").unwrap();
            let exercise_lot = if valid_ex.seconds.is_some() && valid_ex.distance.is_some() {
                ExerciseLot::DistanceAndDuration
            } else if valid_ex.seconds.is_some() {
                ExerciseLot::Duration
            } else if valid_ex.reps.is_some() && valid_ex.weight.is_some() {
                ExerciseLot::RepsAndWeight
            } else if valid_ex.reps.is_some() {
                ExerciseLot::Reps
            } else {
                failed.push(ImportFailedItem {
                    lot: None,
                    step: ImportFailStep::InputTransformation,
                    identifier: format!(
                        "Workout #{}, Set #{}",
                        valid_ex.workout_number, valid_ex.set_order
                    ),
                    error: Some(format!(
                        "Could not determine exercise lot: {}",
                        serde_json::to_string(&valid_ex).unwrap()
                    )),
                });
                continue;
            };
            let existing_exercise = Exercise::find()
                .filter(exercise::Column::Lot.eq(exercise_lot))
                .filter(exercise::Column::Name.eq(&exercise_name))
                .one(&ss.db)
                .await?;
            let generated_id = generate_exercise_id(&exercise_name, exercise_lot, user_id);
            let exercise_id = match existing_exercise {
                Some(db_ex)
                    if db_ex.source == ExerciseSource::Github || db_ex.id == generated_id =>
                {
                    db_ex.id
                }
                _ => match unique_exercises.get(&exercise_name) {
                    Some(mem_ex) => mem_ex.id.clone(),
                    None => {
                        unique_exercises.insert(
                            exercise_name.clone(),
                            exercise::Model {
                                lot: exercise_lot,
                                name: exercise_name,
                                id: generated_id.clone(),
                                ..Default::default()
                            },
                        );
                        generated_id
                    }
                },
            };
            ryot_log!(debug, "Importing exercise with id = {}", exercise_id);
            for set in exercises {
                if let Some(note) = set.notes {
                    notes.push(note);
                }
                let weight = set.weight.map(|d| if d == dec!(0) { dec!(1) } else { d });
                let set_lot = match set.set_order.as_str() {
                    "W" => SetLot::WarmUp,
                    "F" => SetLot::Failure,
                    "D" => SetLot::Drop,
                    _ => SetLot::Normal,
                };
                collected_sets.push(UserWorkoutSetRecord {
                    statistic: WorkoutSetStatistic {
                        weight,
                        reps: set.reps,
                        duration: set.seconds.and_then(|r| r.checked_div(dec!(60))),
                        distance: set.distance.and_then(|d| d.checked_div(dec!(1000))),
                        ..Default::default()
                    },
                    rpe: None,
                    note: None,
                    lot: set_lot,
                    rest_time: None,
                    confirmed_at: None,
                });
            }
            collected_exercises.push(UserExerciseInput {
                notes,
                exercise_id,
                assets: None,
                sets: collected_sets,
            });
        }
        completed.push(ImportCompletedItem::Workout(UserWorkoutInput {
            assets: None,
            start_time: ndt,
            supersets: vec![],
            template_id: None,
            repeated_from: None,
            create_workout_id: None,
            update_workout_id: None,
            exercises: collected_exercises,
            end_time: ndt + workout_duration,
            update_workout_template_id: None,
            name: first_exercise.workout_name.clone(),
            comment: first_exercise.workout_notes.clone(),
        }));
    }
    completed.extend(
        unique_exercises
            .values()
            .cloned()
            .map(ImportCompletedItem::Exercise),
    );
    Ok(ImportResult { failed, completed })
}
