use crate::{
    db::{
        models::{NewPerson, Person},
        schema, Db,
    },
    errors::Error,
};
use diesel::{dsl::insert_into, prelude::*, RunQueryDsl};
use rocket::{fairing::AdHoc, response::status::Created, serde::json::Json};
use schema::people::{self, dsl::*};

#[get("/")]
pub async fn index(db: Db) -> Result<Json<Vec<Person>>, Error<String>> {
    db.run(|connection| {
        schema::people::table
            .select(Person::as_select())
            .get_results(connection)
            .map_err(|_| Error::InternalServerError(Json("on no".to_owned())))
            .map(Json)
    })
    .await
}

#[post("/", data = "<person_input>")]

pub async fn create(
    db: Db,
    person_input: Json<NewPerson>,
) -> Result<Created<Json<Person>>, Error<String>> {
    db.run(move |connection| {
        insert_into(people)
            .values(&*person_input)
            .returning(Person::as_returning())
            .get_result(connection)
            .map_err(|_| Error::UnprocessableData(Json("unprocessable".to_owned())))
            .map(|data| Created::new("/").body(Json(data)))
    })
    .await
}

#[get("/<person_id>")]
pub async fn show(db: Db, person_id: i32) -> Result<Json<Person>, Error<String>> {
    db.run(move |connection| {
        people::table
            .select(Person::as_returning())
            .filter(people::id.eq(person_id))
            .get_result(connection)
            .map_err(|_| Error::NotFound(Json("not found".to_owned())))
            .map(Json)
    })
    .await
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("People routes", |rocket| async {
        rocket.mount("/people", routes![index, create, show])
    })
}
