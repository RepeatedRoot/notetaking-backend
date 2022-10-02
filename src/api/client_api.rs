/* Include dependencies */
use crate::{models::client_model::CafhsClient, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::http::CookieJar;
use rocket::{http::Status, serde::json::Json, State};

/* Create a new client entry in the database, retun the ID */
#[post("/client", data="<new_client>")]
pub fn create_client(db: &State<MongoRepo>, cookies: &CookieJar<'_>, new_client: Json<CafhsClient>) -> Result<Json<InsertOneResult>, Status> {
  let authorised = db.check_auth(cookies);
  
  if authorised {
    let data = CafhsClient { //A structure to hold the client's information
      id: None,
      firstname: new_client.firstname.to_owned(),
      surname: new_client.surname.to_owned(),
      middlenames: new_client.middlenames.to_owned(),
      sex: new_client.sex.clone(),
      address: new_client.address.to_owned(),
      postal_address: new_client.postal_address.to_owned(),
      phone: new_client.phone.to_owned(),
      connections: new_client.connections.clone(),
      notes: Some(db.create_notes().unwrap().inserted_id.as_object_id().expect("Error creating notes for client"))
    };
  
    /* Insert into the database, returning the result, or an error if encountered */
    let user_detail = db.create_client(data);
  
    /* Return the ID or an error */
    match user_detail {
      Ok(client) => Ok(Json(client)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
}

/* Get information about a client given their ID */
#[get("/client/<path>")]
pub fn get_client(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String) -> Result<Json<CafhsClient>, Status> {
  let authorised = db.check_auth(cookies);
  
  if authorised {
    let id = path;  // A string of the ID
    if id.is_empty() {
      return Err(Status::BadRequest); //No ID was present, return an error.
    }
  
    /* Get the client's information from the database */
    let client_detail = db.get_client(&id);
  
    /* Return the client information struct, or an error if generated */
    match client_detail {
      Ok(client) => Ok(Json(client)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
}

/* Get a list of all client's information */
#[get("/clients")]
pub fn get_all_clients(db: &State<MongoRepo>, cookies: &CookieJar<'_>) -> Result<Json<Vec<CafhsClient>>, Status> {
  let authorised = db.check_auth(cookies); //if the web client is authorised to make this request
  
  if authorised { //the web client was authorised, continue
    /* Query the database */
    let clients = db.get_all_clients();
    
    /* Return a vector of client information structs, or an error */
    match clients {
      Ok(clients) => Ok(Json(clients)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else { //the web client was not authorised, return an error
    Err(Status::Forbidden)
  }
}
