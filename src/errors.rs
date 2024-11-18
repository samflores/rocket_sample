use rocket::serde::json::Json;

#[derive(Responder)]
pub enum Error<T> {
    #[response(status = 404, content_type = "json")]
    NotFound(Json<T>),
    #[response(status = 500, content_type = "json")]
    InternalServerError(Json<T>),
    #[response(status = 422, content_type = "json")]
    UnprocessableData(Json<T>),
}
