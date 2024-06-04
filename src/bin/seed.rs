use evaluation_juin_2024_1 as backend;

use std::{fs::File, io::BufReader};

use backend::{
    etablish_connection,
    models::{etape::Etape, points::Points},
    modules::imports::{etape::EtapeCSVDATA, points::PointsCSVData},
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

fn main() -> anyhow::Result<()> {
    let db = etablish_connection();
    let mut pool = db.get()?;
    seed_points(&mut pool)?;
    seed_etapes(&mut pool)?;
    Ok(())
}
