use async_graphql::SimpleObject;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, SimpleObject)]
#[graphql(name = "Songs")]
#[sea_orm(table_name = "songs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub song_id:      i32,
    pub title:        String,
    pub file_url:     String,
    pub track_length: String,
    pub image_url:    Option<String>,
    pub artist_id:    i32,
    pub album_id:     Option<i32>,
    pub created_at:   DateTime,
    pub updated_at:   DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::albums::Entity",
        from       = "Column::AlbumId",
        to         = "super::albums::Column::AlbumId"
    )]
    Albums,
    #[sea_orm(
        belongs_to = "super::artists::Entity",
        from       = "Column::ArtistId",
        to         = "super::artists::Column::ArtistId"
    )]
    Artists,
}

impl ActiveModelBehavior for ActiveModel {}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl Related<super::artists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Artists.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        super::users_songs::Relation::Users.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::users_songs::Relation::Songs.def().rev())
    }
}