use evaluation_juin_2024_1 as backend;

use std::{fs::File, io::BufReader};

use backend::{
    etablish_connection, models::points::Points, modules::imports::points::PointsCSVData,
};
use csv::Reader;
use diesel::{insert_into, prelude::*};

fn main() -> anyhow::Result<()> {
    let buf_read = BufReader::new(File::open(
        "data/données importation juin 2024 - points.csv",
    )?);
    let data = PointsCSVData::read(Reader::from_reader(buf_read))
        .into_iter()
        .map(|p| p.into())
        .collect::<Vec<Points>>();
    let db = etablish_connection();
    let mut pool = db.get()?;
    {
        use backend::schema::points::dsl::*;
        insert_into(points)
            .values(&data)
            .on_conflict(rang)
            .do_nothing()
            .execute(&mut pool)?;
    }
    Ok(())
}
