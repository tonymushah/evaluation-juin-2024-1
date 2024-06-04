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
        self.con
            .build_transaction()
            .run::<TempCoureur, diesel::result::Error, _>(move |con| {
                diesel::update(temps_coureur).set(&temp).execute(con)?;
                temp.attribute_points(con)?;
                Ok(temp)
            })
    }
    pub fn attribute_points(&mut self) -> QueryResult<()> {
        use crate::schema::temps_coureur::dsl::*;

        for mut t in temps_coureur
            .select(TempCoureur::as_select())
            .filter(etape.eq(self.etape))
            .get_results(self.con)?
        {
            let _ = t.attribute_points(self.con);
        }
        Ok(())
    }
    pub fn attribute_points_to_etapes(con: &mut PgConnection, etapes: &[i32]) -> QueryResult<()> {
        for etape in etapes {
            AddPointJoueurModule::new(*etape, con).attribute_points()?;
        }
        Ok(())
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
