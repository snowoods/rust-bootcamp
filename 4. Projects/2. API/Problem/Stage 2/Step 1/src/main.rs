#[macro_use]
extern crate rocket;

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions
#[macro_use]
extern crate log;
extern crate pretty_env_logger;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    pretty_env_logger::init();
    // TODO: Initialize dotenv
    dotenv().ok();

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url. 
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated. 
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.  
    // let recs = todo!();
    let recs = sqlx::query!("SELECT * FROM questions")
        .fetch_all(&pool)
        .await
        .unwrap();

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
