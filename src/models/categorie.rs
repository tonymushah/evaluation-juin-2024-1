use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::categorie;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Identifiable,
    Selectable,
    Insertable,
    Queryable,
    AsChangeset,
    SimpleObject,
    InputObject,
)]
#[graphql(input_name = "CategorieInput")]
#[diesel(table_name = categorie)]
#[diesel(primary_key(id_categorie))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Categorie {
    #[graphql(default_with = "Uuid::new_v4()")]
    pub id_categorie: Uuid,
    #[graphql(name = "designation")]
    pub designation: String,
}
