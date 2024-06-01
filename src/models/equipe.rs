use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::equipe;

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
#[graphql(input_name = "EquipeInput")]
#[diesel(table_name = equipe)]
#[diesel(primary_key(id_equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Equipe {
    pub id_equipe: Uuid,
    pub pseudo: String,
    pub mot_passe: String,
    pub nom: String,
}
