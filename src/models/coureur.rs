use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;
use time::Date;

use crate::schema::coureur;

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
#[graphql(input_name = "CoueurInput")]
#[diesel(table_name = coureur)]
#[diesel(primary_key(numero_dosard))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Coureur {
    pub numero_dosard: i32,
    pub nom: String,
    pub genre: i32,
    pub dtn: Date,
}
