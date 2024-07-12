//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.1

use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::SimpleObject;
use async_trait::async_trait;
use database::UserLot;
use sea_orm::{entity::prelude::*, ActiveValue, FromQueryResult};
use serde::{Deserialize, Serialize};

use crate::{models::media::UserSummary, users::UserPreferences};

fn get_hasher() -> Argon2<'static> {
    Argon2::default()
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, DerivePartialModel,
)]
#[sea_orm(entity = "Entity")]
pub struct UserWithOnlyPreferences {
    pub preferences: UserPreferences,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, FromQueryResult, DerivePartialModel,
)]
#[sea_orm(entity = "Entity")]
pub struct UserWithOnlySummary {
    pub summary: Option<UserSummary>,
}

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
    pub is_demo: Option<bool>,
    pub lot: UserLot,
    #[graphql(skip)]
    pub preferences: UserPreferences,
    #[graphql(skip)]
    pub summary: Option<UserSummary>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::collection::Entity")]
    Collection,
    #[sea_orm(has_many = "super::exercise::Entity")]
    Exercise,
    #[sea_orm(has_many = "super::import_report::Entity")]
    ImportReport,
    #[sea_orm(has_many = "super::integration::Entity")]
    Integration,
    #[sea_orm(has_many = "super::notification_platform::Entity")]
    NotificationPlatform,
    #[sea_orm(has_many = "super::queued_notification::Entity")]
    QueuedNotification,
    #[sea_orm(has_many = "super::review::Entity")]
    Review,
    #[sea_orm(has_many = "super::seen::Entity")]
    Seen,
    #[sea_orm(has_many = "super::user_measurement::Entity")]
    UserMeasurement,
    #[sea_orm(has_many = "super::user_to_collection::Entity")]
    UserToCollection,
    #[sea_orm(has_many = "super::user_to_entity::Entity")]
    UserToEntity,
    #[sea_orm(has_many = "super::workout::Entity")]
    Workout,
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

impl Related<super::notification_platform::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::NotificationPlatform.def()
    }
}

impl Related<super::queued_notification::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QueuedNotification.def()
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

impl Related<super::user_to_collection::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserToCollection.def()
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

impl Related<super::collection::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_to_collection::Relation::Collection.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_to_collection::Relation::User.def().rev())
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
                let password_hash = get_hasher()
                    .hash_password(password.as_bytes(), &salt)
                    .map_err(|_| DbErr::Custom("Unable to hash password".to_owned()))?
                    .to_string();
                self.password = ActiveValue::Set(Some(password_hash));
            }
        }
        Ok(self)
    }
}
