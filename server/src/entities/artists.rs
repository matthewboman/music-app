use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(complex, name = "Artists")]
#[sea_orm(table_name = "artists")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub artist_id:  i32,
    pub email:      String,
    pub username:   String,
    pub image_url:  Option<String>,
    pub bio:        Option<String>,
    pub password:   String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::songs::Entity")]
    Songs,
    #[sea_orm(has_many = "super::albums::Entity")]
    Albums,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Songs.def()
    }
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::artist_tags::Relation::Tags.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::artist_tags::Relation::Tags.def().rev())
    }
}