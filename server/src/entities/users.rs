use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Users")]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub user_id:    i32,
    pub email:      String,
    pub username:   String,
    pub image_url:  Option<String>,
    pub password:   String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_albums::Relation::Albums.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::users_albums::Relation::Users.def().rev())
    }
}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_songs::Relation::Songs.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::users_songs::Relation::Users.def().rev())
    }
}