//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.3

use async_graphql::SimpleObject;
use chrono::NaiveDate;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{migrator::MetadataSource, models::media::MetadataImages};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, SimpleObject)]
#[graphql(name = "Person")]
#[sea_orm(table_name = "person")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub identifier: String,
    pub source: MetadataSource,
    pub created_on: DateTimeUtc,
    pub last_updated_on: DateTimeUtc,
    pub name: String,
    #[graphql(skip)]
    pub images: Option<MetadataImages>,
    pub description: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub death_date: Option<NaiveDate>,
    pub place: Option<String>,
    pub website: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::metadata_to_person::Entity")]
    MetadataToPerson,
}

impl Related<super::metadata_to_person::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::MetadataToPerson.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
