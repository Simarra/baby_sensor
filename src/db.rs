pub mod models;
pub mod schema;

use chrono::Utc;
use diesel::prelude::*;
use dotenv::dotenv;
use models::*;
use schema::statements;
use schema::statements::dsl::*;
use std::env;
use uuid::Uuid;

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

pub fn create_statement(temp_val: &f32) -> String {
    let connection = establish_connection();

    let gen_uuid = Uuid::new_v4().to_hyphenated().to_string();

    let timestmp = i32::from(Utc::now().timestamp())

    let new_statement = NewStatement {
        uuid: &gen_uuid,
        temperature: &temp_val,
        timestamp: &timestmp,
    };

    diesel::insert_into(statements::table)
        .values(&new_statement)
        .execute(&connection)
        .expect("Error saving new post");

    gen_uuid
}
