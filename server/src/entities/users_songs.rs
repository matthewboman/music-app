use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users_songs"
    }
}

#[derive(Clone, Debug, PartialEq, Eq, DeriveModel, DeriveActiveModel)]
// #[sea_orm(table_name = "users_songs")]
pub struct Model {
    pub user_id: i32,
    pub song_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    UserId,
    SongId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    UserId,
    SongId,
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
    Songs,
}

impl ColumnTrait for Column {
    type EntityName = Entity;

    fn def(&self) -> ColumnDef {
        match self {
            Self::UserId => ColumnType::Integer.def(),
            Self::SongId => ColumnType::Integer.def(),
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
            Self::Songs => Entity::belongs_to(super::songs::Entity)
                .from(Column::SongId)
                .to(super::songs::Column::SongId)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}