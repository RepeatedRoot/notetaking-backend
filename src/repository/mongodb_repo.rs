use std::env;
use std::error::Error;

extern crate dotenv;
use crypto::{digest::Digest, sha3::Sha3};
use dotenv::dotenv;

//mongodb api functions
use mongodb::{
    bson,
    bson::{doc, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Collection},
};

use rocket::http::CookieJar;

//Models of available data types
use crate::models::{
    auth_model::AuthInfo,
    client_model::CafhsClient,
    notes_model::{Note, NoteCollection},
    user_model::User,
    workplace_model::Workplace,
};

//A structure to hold connections to each collection in the database
pub struct MongoRepo {
    pub users: Collection<User>,
    pub auth: Collection<AuthInfo>,
    clients: Collection<CafhsClient>,
    workplaces: Collection<Workplace>,
    notes: Collection<NoteCollection>,
}

/* Implementing functions for the MongoRepo structure */
impl MongoRepo {
    /* Hash a password using 256-bit SHA3 */
    pub fn hash_password(password: &String) -> String {
        let mut hasher = Sha3::sha3_256();
        hasher.input_str(password);
        hasher.result_str()
    }

    /* Check if a user is authorised, return true if permitted */
    pub fn check_auth(&self, cookies: &CookieJar<'_>) -> bool {
        let uid = cookies.get_private("user_id");

        match uid {
            Some(id) => {
                let logged_in_id = ObjectId::parse_str(id.value().to_string()).unwrap();
                let possible_user = self.auth.find_one(doc! { "user_id": logged_in_id }, None);

                match possible_user {
                    Ok(Some(_)) => true,
                    _ => false,
                }
            }
            None => false,
        }
    }
    pub fn init() -> Self {
        /* Initialise a new MongoRepo instance */
        dotenv().ok();
        let uri =
            env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment variable!"); //connect to the database

        let client = Client::with_uri_str(uri).unwrap(); //create a new client to the database

        let db = client.database("CAFHS-notetaking"); //Specify the database

        /* A connection to each of the collections */
        let users: Collection<User> = db.collection("users");
        let clients: Collection<CafhsClient> = db.collection("clients");
        let workplaces: Collection<Workplace> = db.collection("workplaces");
        let notes: Collection<NoteCollection> = db.collection("notes");
        let auth: Collection<AuthInfo> = db.collection("auth");

        Self {
            users,
            clients,
            workplaces,
            notes,
            auth,
        } //return a MongoDB struct
    }

    /* Create a new user entry in the database */
    pub fn create_user(&self, new_user: User) -> Result<InsertOneResult, Box<dyn Error>> {
        let new_doc = User {
            //make a document from the input User struct
            id: None,
            firstname: new_user.firstname,
            lastname: new_user.lastname,
            phone: new_user.phone,
            email: new_user.email,
            workplace: new_user.workplace,
            qualification: new_user.qualification,
            password: None,
        };

        let user = self //Insert the document into the database
            .users
            .insert_one(new_doc, None)
            .expect("Error creating user");

        let new_auth = AuthInfo {
            user_id: user.inserted_id.as_object_id().unwrap(),
            password_hash: match new_user.password {
                Some(password) => Self::hash_password(&password),
                None => panic!("No password given"),
            },
        };

        let _auth_result = self
            .auth
            .insert_one(new_auth, None)
            .expect("Error creating auth for user");

        Ok(user) //return the ID of the inserted document
    }

    /* Get the details of a User given their ID */
    pub fn get_user(&self, id: &String) -> Result<User, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap(); //Parse the ID string into an ID object (ObjectId struct)
        let filter = doc! { "_id": obj_id }; //Create a filter document to filter by the ID
        let user_detail = self //Query the database with the filter
            .users
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's details");

        Ok(user_detail.unwrap()) //Return a User struct of the returned details
    }

    /* Update a user's details given new details and their ID */
    pub fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap(); //Parse the ID string into an ID object (ObjectId struct)
        let filter = doc! { "_id": obj_id }; //Create a filter document to filter by the ID
        let new_doc = doc! {  //Create a document to describe the new information
          "$set": {
            "id": new_user.id,
            "firstname": new_user.firstname,
            "lastname": new_user.lastname,
            "phone": new_user.phone,
            "workplace": new_user.workplace,
            "qualification": bson::to_bson(&new_user.qualification)?
          }
        };
        let updated_doc: UpdateResult = self //update the document
            .users
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating User");

        Ok(updated_doc) //return the ID of the updated document
    }

    /* Delete a user given their ID */
    pub fn delete_user(&self, id: &String) -> Result<DeleteResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap(); //Parse the ID string into an ID object (ObjectID struct)
        let filter = doc! { "_id": obj_id }; //Create a filter document to filter by the ID
        let user_detail = self //Delete the document
            .users
            .delete_one(filter, None)
            .ok()
            .expect("Error deleting user");

        Ok(user_detail)
    }

    /* Get a list of all users and their information */
    pub fn get_all_users(&self) -> Result<Vec<User>, Box<dyn Error>> {
        let cursors = self
            .users
            .find(None, None)
            .ok()
            .expect("Error getting list of users");
        let users = cursors.map(|doc| doc.unwrap()).collect();
        Ok(users)
    }

    /* All other methods are implemented in similar ways to those of user manipulation */
    /* Create a new client entry in the database */
    pub fn create_client(
        &self,
        new_client: CafhsClient,
    ) -> Result<InsertOneResult, Box<dyn Error>> {
        let new_doc = CafhsClient {
            id: None,
            firstname: new_client.firstname,
            surname: new_client.surname,
            middlenames: new_client.middlenames,
            sex: new_client.sex,
            address: new_client.address,
            postal_address: new_client.postal_address,
            phone: new_client.phone,
            connections: new_client.connections,
            notes: new_client.notes,
        };

        let client = self
            .clients
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating client");

        Ok(client)
    }

    /* Get the details of a client given their ID */
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

    /* Update a client's details given new details and their ID */
    pub fn update_client(
        &self,
        id: &String,
        new_client: CafhsClient,
    ) -> Result<UpdateResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id).unwrap(); //Parse the ID string into an ID object (ObjectId struct)
        let filter = doc! { "_id": obj_id }; //Create a filter document to filter by the ID
        let new_doc = doc! {  //Create a document to describe the new information
          "$set": {
            "id": new_client.id,
            "firstname": new_client.firstname,
            "middlenames": new_client.middlenames,
            "surname": new_client.surname,
            "sex": bson::to_bson(&new_client.sex)?,
            "address": new_client.address,
            "postal_address": new_client.postal_address,
            "phone": new_client.phone,
            "connections": new_client.connections,
            "notes": new_client.notes,
          }
        };
        let updated_doc: UpdateResult = self //update the document
            .clients
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating User");

        Ok(updated_doc) //return the ID of the updated document
    }

    /* Get a list of all clients in the database */
    pub fn get_all_clients(&self) -> Result<Vec<CafhsClient>, Box<dyn Error>> {
        let cursors = self
            .clients
            .find(None, None)
            .ok()
            .expect("Error getting list of clients");
        let clients = cursors.map(|doc| doc.unwrap()).collect();

        Ok(clients)
    }

    /* Get the details of workplace given it's ID */
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

    /* Get a list of all workplaces in the database */
    pub fn get_all_workplaces(&self) -> Result<Vec<Workplace>, Box<dyn Error>> {
        let cursors = self
            .workplaces
            .find(None, None)
            .ok()
            .expect("Error getting list of Workplaces");
        let workplaces = cursors.map(|doc| doc.unwrap()).collect();

        Ok(workplaces)
    }

    /* Create a new note */
    pub fn create_notes(&self) -> Result<InsertOneResult, Box<dyn Error>> {
        let new_doc = NoteCollection {
            id: None,
            notes: Vec::<Note>::new(),
        };
        let note_collection = self
            .notes
            .insert_one(new_doc, None)
            .expect("Error creating note collection");

        Ok(note_collection)
    }

    /* Get the notes of a client given the ID of the notes entry */
    pub fn get_notes(&self, id: &String) -> Result<Option<NoteCollection>, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id)?;
        let filter = doc! { "_id": obj_id };
        let notes_detail = self
            .notes
            .find_one(filter, None)
            .ok()
            .expect("Error getting notes details");

        Ok(notes_detail)
    }

    /* Add a note to a given collection of notes given the collection's ID */
    pub fn add_note(&self, id: &String, note: Note) -> Result<UpdateResult, Box<dyn Error>> {
        let obj_id = ObjectId::parse_str(id)?;
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
