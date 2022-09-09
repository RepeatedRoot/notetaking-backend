//add external modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user_api::{create_user, get_user, update_user, delete_user, get_all_users};
use api::client_api::{create_client, get_client, get_all_clients};
use api::workplace_api::{get_workplace, get_all_workplaces};
use api::notes_api::{create_notes, get_notes, add_note};
use repository::mongodb_repo::MongoRepo;

use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[get("/")]
fn home() -> &'static str {
    "Hello, world!"
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();

    rocket::build()
    .manage(db)
    .attach(CORS)
    .mount("/", routes![home])
    .mount("/", routes![create_user, get_user, update_user, delete_user, get_all_users])
    .mount("/", routes![create_client, get_client, get_all_clients])
    .mount("/", routes![get_workplace, get_all_workplaces])
    .mount("/", routes![create_notes, get_notes, add_note])
}
