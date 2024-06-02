use async_graphql::SimpleObject;
use diesel::prelude::*;

use crate::view::v_coureur_point;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_coureur_point)]
#[diesel(primary_key(coureur))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CoueurPoint {
    pub coureur: i32,
    pub points: i32,
    pub temps: i32,
}
