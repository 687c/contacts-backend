mod data;
mod db;
mod models;
mod schema;
mod tests;

fn main() {
    db::establish_db_connection();

    println!("Hello, world!");
}
