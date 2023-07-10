use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_db_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").unwrap_or_else(|err| {
        panic!("check your DATABASE_URL env configuration: {err}");
    });
    PgConnection::establish(&database_url).unwrap_or_else(|err| {
        panic!("Error connecting to the database: {err}");
    })
}
