use evaluation_juin_2024_1 as backend;

use std::{fs::File, io::BufReader};

use backend::{
    etablish_connection,
    models::{etape::Etape, points::Points},
    modules::{
        add_point_joueur::AddPointJoueurModule,
        imports::{etape::EtapeCSVDATA, points::PointsCSVData, resultat::ResultatCSV},
    },
    reset::reset_db,
};
use csv::Reader;
use diesel::{insert_into, prelude::*};

fn seed_points(pool: &mut PgConnection) -> anyhow::Result<()> {
    let buf_read = BufReader::new(File::open(
        "data/données importation juin 2024 - points.csv",
    )?);
    let points_data: Vec<Points> = PointsCSVData::read(Reader::from_reader(buf_read))
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Points>>();
    {
        use backend::schema::points::dsl::*;
        insert_into(points)
            .values(&points_data)
            .on_conflict(rang)
            .do_nothing()
            .execute(pool)?;
    }
    Ok(())
}

fn seed_etapes(pool: &mut PgConnection) -> anyhow::Result<()> {
    let buf_read = BufReader::new(File::open(
        "data/données importation juin 2024 - etape.csv",
    )?);
    let etapes_data: Vec<Etape> = EtapeCSVDATA::read(Reader::from_reader(buf_read))
        .into_iter()
        .map(|p| p.into())
        .collect();
    {
        use backend::schema::etape::dsl::*;
        insert_into(etape)
            .values(&etapes_data)
            .on_conflict(rang)
            .do_nothing()
            .execute(pool)?;
    }
    Ok(())
}

fn seed_resultats(pool: &mut PgConnection) -> anyhow::Result<()> {
    let buf_read = BufReader::new(File::open(
        "data/données importation juin 2024 - resultat.csv",
    )?);
    let data = ResultatCSV::read(Reader::from_reader(buf_read));
    let mut etapes = data.iter().map(|r| r.etape_rang).collect::<Vec<_>>();
    etapes.dedup();
    for res in data {
        res.insert(pool)?;
    }
    AddPointJoueurModule::attribute_points_to_etapes(pool, &etapes)?;
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let db = etablish_connection();
    let mut pool = db.get()?;
    reset_db(&mut pool)?;
    seed_points(&mut pool)?;
    seed_etapes(&mut pool)?;
    seed_resultats(&mut pool)?;
    Ok(())
}
