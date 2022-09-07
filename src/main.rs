//add external modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::client_api::{create_client, get_client, get_all_clients};
use api::workplace_api::{get_workplace, get_all_workplaces};
use repository::mongodb_repo::MongoRepo;

#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
    .manage(db)
    .mount("/", routes![home])
    .mount("/", routes![create_user, get_user, update_user, delete_user, get_all_users])
    .mount("/", routes![create_client, get_client, get_all_clients])
    .mount("/", routes![get_workplace, get_all_workplaces])
}
