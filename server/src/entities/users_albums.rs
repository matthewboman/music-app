use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users_albums"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub user_id:  i32,
    pub album_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    UserId,
    AlbumId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    UserId,
    AlbumId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);

    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Users,
    Albums,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::UserId  => ColumnType::Integer.def(),
            Self::AlbumId => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Users => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::UserId)
                .into(),
            Self::Albums => Entity::belongs_to(super::albums::Entity)
                .from(Column::AlbumId)
                .to(super::albums::Column::AlbumId)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}