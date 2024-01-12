use rocket::{serde::json::Json, State};

use crate::{
    models::*,
    persistance::{answers_dao::AnswersDao, questions_dao::QuestionsDao},
};

mod handlers_inner;

use handlers_inner::*;

// outer handler 에러 처리에 사용
#[derive(Responder)]
pub enum APIError {
    #[response(status = 400)]
    BadRequest(String),
    #[response(status = 500)]
    InternalError(String),
}

impl From<HandlerError> for APIError {
    fn from(value: HandlerError) -> Self {
        match value {
            HandlerError::BadRequest(s) => Self::BadRequest(s),
            HandlerError::InternalError(s) => Self::InternalError(s),
        }
    }
}

// ---- CRUD for Questions ----

#[post("/question", data = "<question>")]
pub async fn create_question(
    question: Json<Question>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
// TODO: update return type to be of type `Result`. Use `APIError` for the error case.
//) -> Json<QuestionDetail> { 
) -> Result<Json<QuestionDetail>, APIError> {
    // TODO: Replace the fake data below with a call to `handlers_inner::create_question`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    // Json(QuestionDetail {
    //     question_uuid: "question_uuid".to_owned(),
    //     title: "title".to_owned(),
    //     description: "description".to_owned(),
    //     created_at: "created_at".to_owned(),
    // })

    match handlers_inner::create_question(question.0, questions_dao.inner()).await {
        Ok(q) => Ok(Json(q)),
        Err(err) => Err(err.into()),
    }
}

#[get("/questions")]
pub async fn read_questions(
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
// TODO: update return type to be of type `Result`. Use `APIError` for the error case.
//) -> Json<Vec<QuestionDetail>> { 
) -> Result<Json<Vec<QuestionDetail>>, APIError> { 
    // TODO: Replace the fake data below with a call to `handlers_inner::read_questions`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    // Json(vec![QuestionDetail {
    //     question_uuid: "question_uuid".to_owned(),
    //     title: "title".to_owned(),
    //     description: "description".to_owned(),
    //     created_at: "created_at".to_owned(),
    // }])

    match handlers_inner::read_questions(questions_dao.inner()).await {
        Ok(q) => Ok(Json(q)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/question", data = "<question_uuid>")]
pub async fn delete_question(
    question_uuid: Json<QuestionId>,
    questions_dao: &State<Box<dyn QuestionsDao + Sync + Send>>,
// TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
//) { 
) -> Result<(), APIError> {
    // TODO: Make a call to `handlers_inner::delete_question`.
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.

    match handlers_inner::delete_question(question_uuid.0, questions_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

// ---- CRUD for Answers ----

#[post("/answer", data = "<answer>")]
pub async fn create_answer(
    answer: Json<Answer>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
// TODO: update return type to be of type `Result`. Use `APIError` for the error case.
//) -> Json<AnswerDetail> { 
) -> Result<Json<AnswerDetail>, APIError> {
    // TODO: Replace the fake data below with a call to `handlers_inner::create_answer`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    // Json(AnswerDetail {
    //     answer_uuid: "answer_uuid".to_owned(),
    //     question_uuid: "question_uuid".to_owned(),
    //     content: "content".to_owned(),
    //     created_at: "created_at".to_owned(),
    // })

    match handlers_inner::create_answer(answer.0, answers_dao.inner()).await {
        Ok(q) => Ok(Json(q)),
        Err(err) => Err(err.into()),
    }
}

#[get("/answers", data = "<question_uuid>")]
pub async fn read_answers(
    question_uuid: Json<QuestionId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
// TODO: update return type to be of type `Result`. Use `APIError` for the error case.
//) -> Json<Vec<AnswerDetail>> { 
) -> Result<Json<Vec<AnswerDetail>>, APIError> {
    // TODO: Replace the fake data below with a call to `handlers_inner::read_answers`.
    // Return the result wrapped in JSON in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.
    // Json(vec![AnswerDetail {
    //     answer_uuid: "answer_uuid".to_owned(),
    //     question_uuid: "question_uuid".to_owned(),
    //     content: "content".to_owned(),
    //     created_at: "created_at".to_owned(),
    // }])

    match handlers_inner::read_answers(question_uuid.0, answers_dao.inner()).await {
        Ok(q) => Ok(Json(q)),
        Err(err) => Err(err.into()),
    }
}

#[delete("/answer", data = "<answer_uuid>")]
pub async fn delete_answer(
    answer_uuid: Json<AnswerId>,
    answers_dao: &State<Box<dyn AnswersDao + Send + Sync>>,
// TODO: return `Result` from this function. Use a unit type in the success case and `APIError` in the error case.
//) { 
) -> Result<(), APIError> {
    // TODO: Make a call to `handlers_inner::delete_answer`.
    // Return a unit type in the success case and an `APIError` in the error case.
    // NOTE: You can easily turn `HandlerError` into an `APIError` because of the From trait implementation above.

    match handlers_inner::delete_answer(answer_uuid.0, answers_dao.inner()).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}
