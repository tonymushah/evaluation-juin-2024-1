use async_graphql::{InputValueError, InputValueResult, Scalar, ScalarType, Value};
use diesel::{prelude::*, sql_query, sql_types::Integer};
use regex::Regex;
use uuid::Uuid;

use crate::{models::temps_coureur::TempCoureur, DbPoolConnection};

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

#[derive(Debug, Clone, Copy)]
pub struct TempsCoureur(pub i32);

impl TempsCoureur {
    pub fn parse<I: Into<String>>(i: I) -> crate::Result<Self> {
        let i = i.into();
        let reg = Regex::new(r"(?<heures>[\d+]*):(?<minutes>[\d]*):(?<secondes>[\d]*)")?;
        let Some(captures) = reg.captures(&i) else {
            return Ok(Self(i.parse::<i32>()?));
        };
        let heures = captures
            .name("heures")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "heures",
            )))?
            .as_str()
            .parse::<i32>()?;
        let minutes = captures
            .name("minutes")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "minutes",
            )))?
            .as_str()
            .parse::<i32>()?;
        let secondes = captures
            .name("secondes")
            .ok_or(crate::Error::RegexCaptureNameNotFound(String::from(
                "secondes",
            )))?
            .as_str()
            .parse::<i32>()?;
        Ok(Self(heures * 3600 + minutes * 60 + secondes))
    }
}

#[Scalar]
impl ScalarType for TempsCoureur {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => Ok(Self::parse(s)?),
            Value::Number(i) => Ok(Self(
                i.as_u64().ok_or(crate::Error::TryFromInt)?.try_into()?,
            )),
            _ => Err(InputValueError::expected_type(value)),
        }
    }
    fn to_value(&self) -> Value {
        Value::Number(self.0.into())
    }
}

pub struct AddPointJoueurModule<'a> {
    etape: i32,
    con: &'a mut DbPoolConnection,
}

#[derive(Debug, Clone)]
pub struct AddPointJoueurEntry {
    pub dosard: i32,
    pub temps: TempsCoureur,
    pub point: Option<i32>,
}

impl AddPointJoueurEntry {
    pub fn get_temp_coureur(
        &self,
        etape_: i32,
        con: &mut PgConnection,
    ) -> QueryResult<TempCoureur> {
        let temp_id: Uuid = {
            use crate::view::v_temps_coureur_etape_equipe_coureur::dsl::*;
            v_temps_coureur_etape_equipe_coureur
                .select(equipe_coureur)
                .filter(etape.eq(etape_))
                .filter(coureur.eq(self.dosard))
                .filter(temps.is_not_null())
                .get_result(con)?
        };
        use crate::schema::temps_coureur::dsl::*;
        temps_coureur
            .select(TempCoureur::as_select())
            .filter(id_temps_coureur.eq(temp_id))
            .get_result(con)
    }
}

impl<'a> AddPointJoueurModule<'a> {
    pub fn new(etape: i32, con: &'a mut DbPoolConnection) -> Self {
        Self { etape, con }
    }
    pub fn insert_entry(&mut self, entry: &AddPointJoueurEntry) -> QueryResult<TempCoureur> {
        use crate::schema::temps_coureur::dsl::*;
        let mut temp = entry.get_temp_coureur(self.etape, self.con)?;
        temp.temps = Some(entry.temps.0);
        if entry.point.is_some() {
            temp.points = entry.point;
        };
        self.con
            .build_transaction()
            .run::<TempCoureur, diesel::result::Error, _>(move |con| {
                diesel::update(temps_coureur).set(&temp).execute(con)?;
                temp.attribute_points(con)?;
                Ok(temp)
            })
    }
}

impl TempCoureur {
    pub fn attribute_points(&mut self, con: &mut PgConnection) -> QueryResult<()> {
        let entry = RangEntry::get_rang_from_etape(con, self.etape)?
            .into_iter()
            .find(|e| e.equipe_coureur == self.equipe_coureur)
            .ok_or(diesel::result::Error::NotFound)?;
        let point: i32 = {
            use crate::schema::points::dsl::*;
            points
                .select(valeur)
                .filter(rang.eq(entry.rang))
                .get_result(con)
                .unwrap_or_default()
        };
        let old_points = self.points;
        if old_points.is_none() {
            self.points = Some(point);
            {
                use crate::schema::temps_coureur::dsl::*;
                if let Err(e) = diesel::update(temps_coureur).set(&*self).execute(con) {
                    self.points = old_points;
                    return Err(e);
                }
            }
        }
        Ok(())
    }
}
