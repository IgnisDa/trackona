//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::SimpleObject;
use async_trait::async_trait;
use enum_models::UserLot;
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use user_models::{UserExtraInformation, UserPreferences};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "User")]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub name: String,
    #[graphql(skip)]
    pub password: Option<String>,
    pub oidc_issuer_id: Option<String>,
    pub created_on: DateTimeUtc,
    #[graphql(skip)]
    pub last_login_on: Option<DateTimeUtc>,
    pub lot: UserLot,
    pub is_disabled: Option<bool>,
    pub preferences: UserPreferences,
    #[graphql(skip)]
    pub extra_information: Option<UserExtraInformation>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::access_link::Entity")]
    AccessLink,
    #[sea_orm(has_many = "super::collection::Entity")]
    Collection,
    #[sea_orm(has_many = "super::daily_user_activity::Entity")]
    DailyUserActivity,
    #[sea_orm(has_many = "super::exercise::Entity")]
    Exercise,
    #[sea_orm(has_many = "super::import_report::Entity")]
    ImportReport,
    #[sea_orm(has_many = "super::integration::Entity")]
    Integration,
    #[sea_orm(has_many = "super::metadata::Entity")]
    Metadata,
    #[sea_orm(has_many = "super::notification_platform::Entity")]
    NotificationPlatform,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(has_many = "super::seen::Entity")]
    Seen,
    #[sea_orm(has_many = "super::user_measurement::Entity")]
    UserMeasurement,
    #[sea_orm(has_many = "super::user_notification::Entity")]
    UserNotification,
    #[sea_orm(has_many = "super::user_to_entity::Entity")]
    UserToEntity,
    #[sea_orm(has_many = "super::workout::Entity")]
    Workout,
    #[sea_orm(has_many = "super::workout_template::Entity")]
    WorkoutTemplate,
}

impl Related<super::access_link::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AccessLink.def()
    }
}

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Collection.def()
    }
}

impl Related<super::daily_user_activity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::DailyUserActivity.def()
    }
}

impl Related<super::exercise::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Exercise.def()
    }
}

impl Related<super::import_report::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ImportReport.def()
    }
}

impl Related<super::integration::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Integration.def()
    }
}

impl Related<super::metadata::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Metadata.def()
    }
}

impl Related<super::notification_platform::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NotificationPlatform.def()
    }
}

impl Related<super::review::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Review.def()
    }
}

impl Related<super::seen::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Seen.def()
    }
}

impl Related<super::user_measurement::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserMeasurement.def()
    }
}

impl Related<super::user_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserNotification.def()
    }
}

impl Related<super::user_to_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToEntity.def()
    }
}

impl Related<super::workout::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Workout.def()
    }
}

impl Related<super::workout_template::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::WorkoutTemplate.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, _insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if self.password.is_set() {
            let cloned_password = self.password.clone().unwrap();
            if let Some(password) = cloned_password {
                let salt = SaltString::generate(&mut OsRng);
                let password_hash = Argon2::default()
                    .hash_password(password.as_bytes(), &salt)
                    .map_err(|_| DbErr::Custom("Unable to hash password".to_owned()))?
                    .to_string();
                self.password = ActiveValue::Set(Some(password_hash));
            }
        }
        Ok(self)
    }
}
