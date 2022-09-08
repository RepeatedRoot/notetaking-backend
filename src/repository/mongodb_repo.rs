use std::env;
use std::error::Error;

extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
  bson,
  bson::{oid::ObjectId, doc},
  results::{InsertOneResult, UpdateResult, DeleteResult},
  sync::{Client, Collection}
};
use crate::models::{user_model::User, client_model::CafhsClient, workplace_model::Workplace, notes_model::{Note, NoteCollection}};

pub struct MongoRepo {
  users: Collection<User>,
  clients: Collection<CafhsClient>,
  workplaces: Collection<Workplace>,
  notes: Collection<NoteCollection>
}

impl MongoRepo {
  pub fn init() -> Self {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment variable!");

    let client = Client::with_uri_str(uri).unwrap();
    let db = client.database("CAFHS-notetaking");
    let users: Collection<User> = db.collection("users");
    let clients: Collection<CafhsClient> = db.collection("clients");
    let workplaces: Collection<Workplace> = db.collection("workplace");
    let notes: Collection<NoteCollection> = db.collection("notes");

    Self { users, clients, workplaces, notes }
  }

  pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Box<dyn Error>> {
    let new_doc = User {
      id: None,
      firstname: new_user.firstname,
      lastname: new_user.lastname,
      phone: new_user.phone,
      workplace: new_user.workplace,
      qualification: new_user.qualification
    };
    let user = self
      .users
      .insert_one(new_doc, None)
      .expect("Error creating user");
    
    Ok(user)
  }

  pub fn get_user(&self, id: &String) -> Result<User, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let user_detail = self
      .users
      .find_one(filter, None)
      .ok()
      .expect("Error getting user's details");
    
    Ok(user_detail.unwrap())
  }

  pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let new_doc = doc! {
      "$set": {
        "id": new_user.id,
        "firstname": new_user.firstname,
        "lastname": new_user.lastname,
        "phone": new_user.phone,
        "workplace": new_user.workplace,
        "qualification": bson::to_bson(&new_user.qualification)?
      }
    };
    let updated_doc: UpdateResult = self
      .users
      .update_one(filter, new_doc, None)
      .ok()
      .expect("Error updating User");

    Ok(updated_doc)
  }

  pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let user_detail = self
      .users
      .delete_one(filter, None)
      .ok()
      .expect("Error deleting user");
    
    Ok(user_detail)
  }

  pub fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
    let cursors = self
      .users
      .find(None, None)
      .ok()
      .expect("Error getting list of users");
    let users = cursors.map(|doc| doc.unwrap()).collect();
    Ok(users)
  }

  pub fn create_client(&self, new_client: CafhsClient) -> Result<InsertOneResult, Box<dyn Error>> {
    let new_doc = CafhsClient {
      id: None,
      firstname: new_client.firstname,
      surname: new_client.surname,
      middlenames: new_client.middlenames,
      sex: new_client.sex,
      address: new_client.address,
      postal_address: new_client.postal_address,
      phone: new_client.phone,
      connections: new_client.connections
    };

    let client = self
      .clients
      .insert_one(new_doc, None) 
      .ok()
      .expect("Error creating client");

    Ok(client)
  }

  pub fn get_client(&self, id: &String) -> Result<CafhsClient, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let client_detail = self
      .clients
      .find_one(filter, None)
      .ok()
      .expect("Error getting client's details");
    
    Ok(client_detail.unwrap())
  }

  pub fn get_all_clients(&self) -> Result<Vec<CafhsClient>, Box<dyn Error>> {
    let cursors = self
      .clients
      .find(None, None)
      .ok()
      .expect("Error getting list of clients");
    let clients = cursors.map(|doc| doc.unwrap()).collect();

    Ok(clients)
  }

  pub fn get_workplace(&self, id: &String) -> Result<Workplace, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let workplace_detail = self
      .workplaces
      .find_one(filter, None)
      .ok()
      .expect("Error getting workplace details");

    Ok(workplace_detail.unwrap())
  }

  pub fn get_all_workplaces(&self) -> Result<Vec<Workplace>, Box<dyn Error>> {
    let cursors = self
      .workplaces
      .find(None, None)
      .ok()
      .expect("Error getting list of Workplaces");
    let workplaces = cursors.map(|doc| doc.unwrap()).collect();

    Ok(workplaces)
  }

  pub fn create_notes(&self) -> Result<InsertOneResult, Box<dyn Error>> {
    let new_doc = NoteCollection {
      id: None,
      notes: Vec::<Note>::new()
    };
    let note_collection = self
      .notes
      .insert_one(new_doc, None)
      .expect("Error creating note collection");
    
    Ok(note_collection)
  }

  pub fn get_notes(&self, id: &String) -> Result<NoteCollection, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let notes_detail = self
      .notes
      .find_one(filter, None)
      .ok()
      .expect("Error getting notes details");
    
    Ok(notes_detail.unwrap())
  }

  pub fn add_note(&self, id: &String, note: Note) -> Result<UpdateResult, Box<dyn Error>> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let new_doc = doc! {
      "$push": {
        "notes": bson::to_bson(&note)?
      }
    };

    let updated_doc: UpdateResult = self
      .notes
      .update_one(filter, new_doc, None)
      .ok()
      .expect("Error updating notes");
    
    Ok(updated_doc)
  }
}