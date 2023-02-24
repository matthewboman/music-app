use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "album_tags"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub album_id: i32,
    pub tag_id:   i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    AlbumId,
    TagId,
}


#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    AlbumId,
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
    Albums,
    Tags,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self)  -> ColumnDef {
        match self {
            Self::AlbumId => ColumnType::Integer.def(),
            Self::TagId   => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Albums => Entity::belongs_to(super::albums::Entity)
                .from(Column::AlbumId)
                .to(super::albums::Column::AlbumId)
                .into(),
            Self::Tags => Entity::belongs_to(super::tags::Entity)
                .from(Column::TagId)
                .to(super::tags::Column::TagId)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}