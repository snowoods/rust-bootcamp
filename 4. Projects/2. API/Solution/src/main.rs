#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;

//  - 웹 개발에서는 API 개발을 Endpoints 라고 하며 같은 것으로 취급하는 경향이 있는 것 같다.
//  - 웹 클라이언트 입장에서 API 사용은 Endpoints 서버에 접근하는 것이라서 그런 경향이 있는 것 같다.

// < 개발 순서 >
// 
// 구조
//  - Client -> Rocket outer handler -> Inner Handler -> DAO -> PgSQL
// 
// API/Endpoints & Models (Stage1-Step 1,2)
// Rocket outer handler
//  - 로컬 동작하는 한 세트 설정 (docker-pgql, rocket)
//   (https://github.com/snowoods/rust-bootcamp/tree/dev/4.%20Projects/2.%20API/Problem/Stage%201)
//  - main.rs, models.rs, cors.rs, handlers/mod.rs
//  - dependencies (serde, tokio, rocket)

// Persistence (models & connection) (Stage2-Step1)
//  - Docker, PgSQL 연결 (main.rs) - 로그 출력으로 쿼리 결과 출력
//   (https://github.com/snowoods/rust-bootcamp/tree/dev/4.%20Projects/2.%20API/Problem/Stage%202)
//  - .cargo/config.toml, migrations/...sql, 
//  - dependencies (sqlx, dotenvy, log, pretty_env_logger)

// DAO (Data Access Object) CRUD (Stage2-Step2)
// pgsql DAO에서 CRUD 작성
//  - persistance/answers_dao.rs,questions_dao.rs
//  - dependencies (thiserror, async_trait)

// Inner Handler 만들기 (Stage3-Step1)
// DAO를 사용하여 DB를 활용하는 로직 구현.
//  - handlers/handlers_inner.rs

// DAO를 Rocket State에 연결하기 (Stage3-Step2)
// main.rs에서 dao를 rocket에 연결한다. manage() 사용.
// main.rs(questions_dao, answers_dao), handlers/mod.rs

// Rocket outer handler에서 inner handler 사용. (Stage3-Step3)
//  - handlers/mod.rs, APIError, 

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool);

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
        .manage(Box::new(questions_dao) as Box<dyn QuestionsDao + Send + Sync>)
        .manage(Box::new(answers_dao) as Box<dyn AnswersDao + Send + Sync>)
}
