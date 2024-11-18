pub mod models;
pub mod schema;

use rocket::fairing::AdHoc;
use rocket_sync_db_pools::{database, diesel};

#[database("main_db")]
pub struct Db(diesel::PgConnection);

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel PostgreSQL Stage", |rocket| async {
        rocket.attach(Db::fairing())
    })
}
