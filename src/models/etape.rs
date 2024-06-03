pub mod coureur_point;
pub mod equipe_point;

use async_graphql::{InputObject, SimpleObject};
use bigdecimal::BigDecimal;
use diesel::prelude::*;
use time::PrimitiveDateTime;

use crate::schema::etape;

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
#[graphql(input_name = "EtapeInput")]
#[diesel(table_name = etape)]
#[diesel(primary_key(rang))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Etape {
    pub rang: i32,
    pub longueur: BigDecimal,
    pub nom: String,
    pub nb_coureur_par_equipe: i32,
    pub depart: PrimitiveDateTime,
    pub finished: Option<PrimitiveDateTime>,
}
