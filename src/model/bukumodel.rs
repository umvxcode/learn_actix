use sea_orm::entity::prelude::*;
use serde::{Serialize, Deserialize};  // ✅ Import Serde

#[derive(Clone, Debug, Serialize, Deserialize, DeriveEntityModel)]  // ✅ Add Serialize & Deserialize
#[sea_orm(table_name = "buku")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub tahun_terbit: i32,
    pub penulis: String,
    pub judul: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}  // ✅ Add DeriveRelation

impl ActiveModelBehavior for ActiveModel {}
