use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::coureur_categorie;

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
#[graphql(input_name = "CoueurCategorieInput")]
#[diesel(table_name = coureur_categorie)]
#[diesel(primary_key(id_coureur_categorie))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CoureurCategorie {
    pub id_coureur_categorie: Uuid,
    pub categorie: Uuid,
    pub coureur: i32,
}
