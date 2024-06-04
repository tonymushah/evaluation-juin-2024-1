use async_graphql::SimpleObject;
use diesel::prelude::*;
use uuid::Uuid;

use crate::view::v_equipe_point_etape;

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Identifiable, Selectable, Queryable, SimpleObject,
)]
#[diesel(table_name = v_equipe_point_etape)]
#[diesel(primary_key(equipe, etape))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EquipePointEtape {
    pub equipe: Uuid,
    pub points: i64,
    pub temps: i64,
    pub etape: i32,
}
