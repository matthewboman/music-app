use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "Tags")]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub tag_id:     i32,
    pub name:       String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::artists::Entity> for Entity {
    fn to() -> RelationDef {
        super::artist_tags::Relation::Artists.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::artist_tags::Relation::Artists.def().rev())
    }
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_tags::Relation::Albums.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_tags::Relation::Albums.def().rev())
    }
}