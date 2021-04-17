pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use models::*;
use schema::statements;
use schema::statements::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_statements() -> Vec<Statement> {
    let connection = establish_connection();
    let results = statements
        .filter(timestamp.gt(0))
        .limit(5)
        .load::<Statement>(&connection)
        .expect("Error loading statements");

    println!("Displaying {} posts", results.len());

    for statement in &results {
        println!("{}", statement.id);
    }

    results
}
