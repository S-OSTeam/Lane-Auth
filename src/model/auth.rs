//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.5

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "auth")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub account_status: Option<String>,
    pub role: Vec<String>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(created_at)]
    pub created_at: Option<DateTimeWithTimeZone>,
    #[sea_orm(updated_at)]
    pub updated_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::auth_sns::Entity")]
    AuthSns,
}

impl Related<super::auth_sns::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AuthSns.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
