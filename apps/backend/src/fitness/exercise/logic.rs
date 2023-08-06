use sea_orm::{prelude::DateTimeUtc, FromJsonQueryResult};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq)]
#[serde(tag = "t", content = "d")]
pub enum SetStatistic {
    Duration(u16),
    DistanceAndDuration(u16, u16),
    RepsAndWeight(u16, u16),
}

#[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq)]
pub enum SetPersonalBest {
    Weight,
    OneRm,
    Volume,
}

pub mod done {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq)]
    pub struct DoneSetRecord {
        pub statistic: SetStatistic,
        pub personal_bests: Vec<SetPersonalBest>,
    }

    #[derive(Debug, FromJsonQueryResult, Clone, Serialize, Deserialize, Eq, PartialEq)]
    pub struct DoneTotal {
        /// The number of personal bests achieved.
        pub personal_bests: u16,
        pub weight: u32,
        pub reps: u32,
        pub active_duration: u32,
    }

    #[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq)]
    pub struct DoneExercise {
        pub idx: u16,
        pub exercise_id: i32,
        pub sets: Vec<DoneSetRecord>,
        pub notes: Vec<String>,
        pub rest_time: Option<u16>,
        pub total: DoneTotal,
    }

    #[derive(Clone, Debug, Deserialize, Serialize, FromJsonQueryResult, Eq, PartialEq)]
    struct DoneWorkout {
        /// A unique identifier for this workout.
        pub identifier: String,
        pub name: String,
        pub start_time: DateTimeUtc,
        pub end_time: DateTimeUtc,
        pub exercises: Vec<DoneExercise>,
        /// Each grouped superset of exercises will be in a vector.
        pub supersets: Vec<Vec<u16>>,
        pub total: DoneTotal,
    }
}

pub mod in_progress {
    use super::*;

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InProgressSetRecord {
        pub statistic: SetStatistic,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InProgressExercise {
        pub exercise_id: i32,
        pub sets: Vec<InProgressSetRecord>,
        pub notes: Vec<String>,
        pub rest_time: Option<u16>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    struct InProgressWorkout {
        pub name: String,
        pub start_time: DateTimeUtc,
        pub end_time: DateTimeUtc,
        pub exercises: Vec<InProgressExercise>,
        /// Each grouped superset of exercises will be in a vector.
        pub supersets: Vec<Vec<u16>>,
    }

    impl InProgressWorkout {
        fn new(name: String, start_time: DateTimeUtc, end_time: DateTimeUtc) -> Self {
            Self {
                name,
                start_time,
                end_time,
                exercises: vec![],
                supersets: vec![],
            }
        }
    }
}
