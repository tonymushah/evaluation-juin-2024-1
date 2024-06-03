use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::view::v_equipe_point;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_equipe_point)]
#[diesel(primary_key(equipe))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipePoint {
    pub equipe: i32,
    pub points: BigDecimal,
    pub temps: BigDecimal,
}
