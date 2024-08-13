//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use async_graphql::SimpleObject;
use async_trait::async_trait;
use enums::NotificationPlatformLot;
use nanoid::nanoid;
use sea_orm::{entity::prelude::*, ActiveValue};
use user_models::NotificationPlatformSpecifics;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, SimpleObject)]
#[sea_orm(table_name = "notification_platform")]
#[graphql(name = "NotificationPlatform")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: String,
    pub lot: NotificationPlatformLot,
    pub created_on: DateTimeWithTimeZone,
    pub is_disabled: Option<bool>,
    #[graphql(skip)]
    pub platform_specifics: NotificationPlatformSpecifics,
    pub description: String,
    #[graphql(skip)]
    pub user_id: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.id = ActiveValue::Set(format!("ntf_{}", nanoid!(12)));
        }
        Ok(self)
    }
}
