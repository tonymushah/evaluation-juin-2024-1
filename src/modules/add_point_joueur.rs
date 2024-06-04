pub mod rang_entry;
pub mod temps_coureur;

use self::rang_entry::RangEntry;

pub use self::temps_coureur::TempsCoureur;

use diesel::prelude::*;
use uuid::Uuid;

use crate::models::temps_coureur::TempCoureur;

pub struct AddPointJoueurModule<'a> {
    etape: i32,
    con: &'a mut PgConnection,
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
    pub fn new(etape: i32, con: &'a mut PgConnection) -> Self {
        Self { etape, con }
    }
    pub fn insert_entry(&mut self, entry: &AddPointJoueurEntry) -> QueryResult<TempCoureur> {
        use crate::schema::temps_coureur::dsl::*;
        let mut temp = entry.get_temp_coureur(self.etape, self.con)?;
        temp.temps = Some(entry.temps.0);
        if entry.point.is_some() {
            temp.points = entry.point;
        };

        let tmp = self
            .con
            .build_transaction()
            .run::<TempCoureur, diesel::result::Error, _>(move |con| {
                diesel::update(temps_coureur)
                    .set(&temp)
                    .returning(TempCoureur::as_returning())
                    .get_result(con)
            })?;
        self.attribute_points()?
            .into_iter()
            .find(|t| t.id_temps_coureur == tmp.id_temps_coureur)
            .ok_or(diesel::result::Error::NotFound)
    }
    pub fn attribute_points(&mut self) -> QueryResult<Vec<TempCoureur>> {
        use crate::schema::temps_coureur::dsl::*;

        Ok(temps_coureur
            .select(TempCoureur::as_select())
            .filter(etape.eq(self.etape))
            .get_results::<TempCoureur>(self.con)?
            .into_iter()
            .flat_map(|tm| tm.attribute_points(self.con))
            .collect())
    }
}

pub fn attribute_points_to_etapes(
    con: &mut PgConnection,
    etapes: &[i32],
) -> QueryResult<Vec<TempCoureur>> {
    let mut tmp: Vec<TempCoureur> = Vec::new();
    for etape in etapes {
        tmp.append(&mut AddPointJoueurModule::new(*etape, con).attribute_points()?);
    }
    Ok(tmp)
}

impl TempCoureur {
    pub fn attribute_points(self, con: &mut PgConnection) -> QueryResult<Self> {
        let entrys = RangEntry::get_rang_from_etape(con, self.etape)?;
        let entry = entrys
            .into_iter()
            .find(|e| e.equipe_coureur == self.equipe_coureur && e.etape == self.etape)
            .ok_or(diesel::result::Error::NotFound)?;
        {
            use crate::schema::temps_coureur::dsl::*;
            diesel::update(temps_coureur)
                .set(points.eq(Some(entry.points.unwrap_or_default())))
                .filter(id_temps_coureur.eq(self.id_temps_coureur))
                .returning(Self::as_returning())
                .get_result(con)
        }
    }
}
