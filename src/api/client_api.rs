/* Include dependencies */
use crate::{models::client_model::CafhsClient, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::http::CookieJar;
use rocket::{http::Status, serde::json::Json, State};

/* Create a new client entry in the database, retun the ID */
#[post("/client", data = "<new_client>")]
pub fn create_client(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    new_client: Json<CafhsClient>,
) -> Result<Json<InsertOneResult>, Status> {
    let authorised = db.check_auth(cookies); //Check authorisation status

    if authorised { //Authorised
        //A structure to hold the client's information
        let data = CafhsClient {
            id: None,
            firstname: new_client.firstname.to_owned(),
            surname: new_client.surname.to_owned(),
            middlenames: new_client.middlenames.to_owned(),
            sex: new_client.sex.clone(),
            address: new_client.address.to_owned(),
            postal_address: new_client.postal_address.to_owned(),
            phone: new_client.phone.to_owned(),
            connections: new_client.connections.clone(),
            notes: Some( //create a new notes entry for the new account
                db.create_notes()
                    .unwrap()
                    .inserted_id
                    .as_object_id()
                    .expect("Error creating notes for client"),
            ),
        };

        /* Insert into the database, returning the result, or an error if encountered */
        let user_detail = db.create_client(data);

        /* Return the ID or an error */
        match user_detail {
            Ok(client) => Ok(Json(client)),
            Err(_) => Err(Status::InternalServerError),
        }
    } else { //Not authorised
        Err(Status::Forbidden)
    }
}

/* Get information about a client given their ID */
#[get("/client/<path>")]
pub fn get_client(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
) -> Result<Json<CafhsClient>, Status> {
    let authorised = db.check_auth(cookies); //Check authorisation

    if authorised { //Authorised
        let id = path; // A string of the ID
        if id.is_empty() {
            return Err(Status::BadRequest); //No ID was present, return an error.
        }

        /* Get the client's information from the database */
        let client_detail = db.get_client(&id);

        /* Return the client information struct, or an error if generated */
        match client_detail {
            Ok(client) => Ok(Json(client)),
            Err(_) => Err(Status::InternalServerError),
        }
    } else { //Not authorised
        Err(Status::Forbidden)
    }
}

/* Update a client's information given their ID */
#[put("/client/<path>", data="<new_client>")]
pub fn update_client(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String, new_client: Json<CafhsClient>) -> Result<Json<CafhsClient>, Status> {
  let authorised = db.check_auth(cookies); //Check authorisation
  
  if authorised {
    //let new_client = new_client.into_inner(); //unwrap the new_client Structure from the validation wrapping
    
    /* Check if ID was passed to endpoint */
    let id = path;
    if id.is_empty() {
      return Err(Status::BadRequest); //There was no ID, return an error
    }

    //Deserialise data
    let data = CafhsClient {
      id: Some(ObjectId::parse_str(&id).unwrap()),
      firstname: new_client.firstname.to_owned(),
      middlenames: new_client.middlenames.to_owned(),
      surname: new_client.surname.to_owned(),
      sex: new_client.sex.clone(),
      address: new_client.address.to_owned(),
      postal_address: new_client.postal_address.to_owned(),
      phone: new_client.phone.to_owned(),
      connections: new_client.connections.to_owned(),
      notes: new_client.notes.to_owned()
    };
  
    let update_result = db.update_client(&id, data); //Update information
  
    match update_result {
      Ok(update) => { //Update operation occurred
        if update.matched_count == 1 { //Updated an entry in the database
          let updated_client_info = db.get_client(&id); //Get updated entry
          match updated_client_info {
            Ok(client) => Ok(Json(client)), //Entry was returned, serialise to JSON and return
            Err(_) => Err(Status::InternalServerError)  //There was no entry returned, return an error
          }
        } else { //No entry was updated
          Err(Status::NotFound) //Return error
        }
      },
      Err(_) => Err(Status::InternalServerError) //There was an error when trying to update the information
    }
  } else { //Not authorised
    Err(Status::Forbidden) //return an error
  }
}

/* Get a list of all client's information */
#[get("/clients")]
pub fn get_all_clients(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<CafhsClient>>, Status> {
    let authorised = db.check_auth(cookies); //if the web client is authorised to make this request

    if authorised {
        //the web client was authorised, continue
        /* Query the database */
        let clients = db.get_all_clients();

        /* Return a vector of client information structs, or an error */
        match clients {
            Ok(clients) => Ok(Json(clients)),
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        //the web client was not authorised, return an error
        Err(Status::Forbidden)
    }
}
