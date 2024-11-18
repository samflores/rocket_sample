use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use rocket::serde::Serialize;
use serde::Deserialize;

#[derive(Insertable, Queryable, Identifiable, Selectable, Serialize)]
#[diesel(table_name = crate::db::schema::people)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub born_at: Option<NaiveDate>,
    pub updated_at: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::db::schema::people)]
pub struct NewPerson {
    name: String,
    born_at: Option<NaiveDate>,
}
