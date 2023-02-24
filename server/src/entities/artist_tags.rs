use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "artist_tags"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub artist_id: i32,
    pub tag_id:    i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    ArtistId,
    TagId,
}


#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    ArtistId,
    TagId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Artists,
    Tags,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::ArtistId => ColumnType::Integer.def(),
            Self::TagId    => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Artists => Entity::belongs_to(super::artists::Entity)
                .from(Column::ArtistId)
                .to(super::artists::Column::ArtistId)
                .into(),
            Self::Tags => Entity::belongs_to(super::tags::Entity)
                .from(Column::TagId)
                .to(super::tags::Column::TagId)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}