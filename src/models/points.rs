use async_graphql::{InputObject, SimpleObject};
use diesel::prelude::*;

use crate::schema::points;

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
#[graphql(input_name = "PointsInput")]
#[diesel(table_name = points)]
#[diesel(primary_key(rang))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Points {
    pub rang: i32,
    pub valeur: i32,
}
