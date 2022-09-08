use crate::{models::client_model::CafhsClient, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/client", data="<new_client>")]
pub fn create_client(db: &State<MongoRepo>, new_client: Json<CafhsClient>) -> Result<Json<InsertOneResult>, Status> {
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
    notes: Some(db.create_notes().unwrap().inserted_id.as_object_id().expect("Error creating notes for client"))
  };

  let user_detail = db.create_client(data);

  match user_detail {
    Ok(client) => Ok(Json(client)),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[get("/client/<path>")]
pub fn get_client(db: &State<MongoRepo>, path: String) -> Result<Json<CafhsClient>, Status> {
  let id = path;
  if id.is_empty() {
    return Err(Status::BadRequest);
  }
  let client_detail = db.get_client(&id);

  match client_detail {
    Ok(client) => Ok(Json(client)),
    Err(_) => Err(Status::InternalServerError)
  }
}
#[get("/clients")]
pub fn get_all_clients(db: &State<MongoRepo>) -> Result<Json<Vec<CafhsClient>>, Status> {
  let clients = db.get_all_clients();
  
  match clients {
    Ok(clients) => Ok(Json(clients)),
    Err(_) => Err(Status::InternalServerError)
  }
}
