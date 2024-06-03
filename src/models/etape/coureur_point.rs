use async_graphql::SimpleObject;
use bigdecimal::BigDecimal;
use diesel::prelude::*;

use crate::view::v_coureur_point_etape;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_coureur_point_etape)]
#[diesel(primary_key(coureur, etape))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CoueurPointEtape {
    pub coureur: i32,
    pub points: BigDecimal,
    pub temps: BigDecimal,
    pub etape: i32,
}
