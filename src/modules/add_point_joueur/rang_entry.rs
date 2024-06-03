use diesel::{prelude::*, sql_query, sql_types::Integer};
use uuid::Uuid;

diesel::table! {
    rang_entrys (rang) {
        rang -> Integer,
        etape -> Integer,
        coureur -> Integer,
        equipe -> Uuid,
        temps -> Integer,
        equipe_coureur -> Uuid,
        points -> Integer
    }
}

#[derive(Debug, Clone, Copy, QueryableByName)]
pub struct RangEntry {
    pub rang: i32,
    pub etape: i32,
    pub coureur: i32,
    pub equipe: Uuid,
    pub temps: i32,
    pub equipe_coureur: Uuid,
    pub points: i32,
}

const RANG_SQL: &str = "select rank() over ( order by temps asc ) rang, etape, coureur, equipe, equipe_coureur, points from v_temps_coureur_etape_equipe_coureur group by etape, temps, coureur, equipe, equipe_coureur, points";

impl RangEntry {
    pub fn get_rang_from_etape(con: &mut PgConnection, etape: i32) -> QueryResult<Vec<RangEntry>> {
        let query = sql_query(format!("{RANG_SQL} where etape = ? and temps is not null"))
            .bind::<Integer, _>(etape);
        query.get_results(con)
    }
}
