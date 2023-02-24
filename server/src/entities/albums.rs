use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "Albums")]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub album_id:   i32,
    pub title:      String,
    pub image_url:  Option<String>,
    pub details:    Option<String>,
    pub artist_id:  i32,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::artists::Entity",
        from       = "Column::ArtistId",
        to         = "super::artists::Column::ArtistId"
    )]
    Artists,
    #[sea_orm(has_many = "super::songs::Entity")]
    Songs,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::artists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artists.def()
    }
}

impl Related<super::songs::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Songs.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_albums::Relation::Users.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::users_albums::Relation::Albums.def().rev())
    }
}

impl Related<super::tags::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_tags::Relation::Tags.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_tags::Relation::Tags.def().rev())
    }
}