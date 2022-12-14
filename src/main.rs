//add external modules
mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

/* Include endpoint handlers */
use api::auth_api::{login, logout, user_id};
use api::client_api::{create_client, get_all_clients, get_client, update_client};
use api::notes_api::{add_note, create_notes, get_notes};
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user};
use api::workplace_api::{get_all_workplaces, get_workplace};

use repository::mongodb_repo::MongoRepo;

/* Rocket API handlers */
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::{relative, FileServer};
use rocket::http::Header;
use rocket::{Request, Response};

/* To hold CORS header information */
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        //Information about the CORS implementation for the Fairing trait
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    /* set headers for the response payload */
    async fn on_response<'r>(&self, _req: &'r Request<'_>, res: &mut Response<'r>) {
        res.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "192.168.0.20:8000",
        ));
        res.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE",
        ));
        res.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        res.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

/* launching the rocket server */
#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init(); //Initialise the connection to the MongoDB database

    /* Add endpoints and states to rocket instance */
    rocket::build()
        .manage(db) //Connection to the database
        .attach(CORS) //CORS policy handler
        .mount(
            "/",
            routes![
                create_user,
                get_user,
                update_user,
                delete_user,
                get_all_users
            ],
        ) //User data manipulation
        .mount(
            "/",
            routes![create_client, get_client, update_client, get_all_clients],
        ) //Client data manipulation
        .mount("/", routes![get_workplace, get_all_workplaces]) //Workplaces
        .mount("/", routes![create_notes, get_notes, add_note]) //Managing notes
        .mount("/", routes![login, logout, user_id]) //Authentication
        .mount("/", FileServer::from(relative!("frontend"))) //Serve the user-interface files
}
