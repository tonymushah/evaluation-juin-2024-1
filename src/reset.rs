use std::env;

use diesel::{dsl::sql, sql_types::Untyped, Connection, RunQueryDsl};

use crate::{modules::generate_categories::SystemCategories, DbPoolConnection};

pub fn reset_db(con: &mut DbPoolConnection) -> crate::Result<()> {
    let user = env::var("DBUSER")?;
    sql::<Untyped>(format!("SELECT truncate_tables('{user}');").as_str()).execute(con)?;
    con.transaction(|con| SystemCategories::seed(con))?;
    Ok(())
}
