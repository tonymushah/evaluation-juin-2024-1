use diesel::{prelude::*, sql_query};
use uuid::Uuid;

diesel::table! {
    rang_entrys (rang) {
        rang -> BigInt,
        etape -> Integer,
        coureur -> Integer,
        equipe -> Uuid,
        equipe_coureur -> Uuid,
        points -> Nullable<Integer>
    }
}

#[derive(Debug, Clone, Copy, QueryableByName)]
pub struct RangEntry {
    pub rang: i64,
    pub etape: i32,
    pub coureur: i32,
    pub equipe: Uuid,
    pub equipe_coureur: Uuid,
    pub points: Option<i32>,
}

impl RangEntry {
    pub fn get_rang_from_etape(con: &mut PgConnection, etape: i32) -> QueryResult<Vec<RangEntry>> {
        let query = sql_query(format!(include_str!("./rang.sql"), etape));
        query.get_results(con)
    }
}
