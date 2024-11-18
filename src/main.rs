use rocket::Request;

mod db;
mod errors;
mod people;

#[macro_use]
extern crate rocket;

#[catch(404)]
fn not_found(_req: &Request) -> String {
    todo!()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found])
        .attach(people::stage()) //
        .attach(db::stage()) //
}
