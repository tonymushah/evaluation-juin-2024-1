use std::env;

use diesel::{dsl::sql, sql_types::Untyped, RunQueryDsl};

use crate::DbPoolConnection;

pub fn reset_db(con: &mut DbPoolConnection) -> crate::Result<()> {
    let user = env::var("DBUSER")?;
    sql::<Untyped>(format!("SELECT truncate_tables('{user}');").as_str()).execute(con)?;
    Ok(())
}
