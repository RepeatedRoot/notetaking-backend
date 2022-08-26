use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, oid::ObjectId},
    {Client, Collection}
};
use chrono::{Utc};
use std::error::Error;
use bson::{Bson, document::Document};

const DB_NAME: &str = "CAFHS-notetaking";
const COLL_NAME: &str = "clients";
 
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Sex {
    Male,
    Female,
    Other
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub surname: String,
    pub given_names: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_names: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub dob: chrono::DateTime<Utc>,
    pub sex: Sex,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    pub phone: u32,
}

#[derive(Debug, Clone)]
pub struct ClientMgr {
    pub coll: Collection<Document>
}

/* Functions for user management */
impl ClientMgr {
    pub async fn new(db_uri: &str) -> Result<Self, Box<dyn Error>> {
        let client: Client = Client::with_uri_str(db_uri).await?;
        let coll: Collection<Document> = client.database(DB_NAME).collection(COLL_NAME);

        Ok(Self { coll } )
    }

    pub async fn db_insert_client(&self, client: &User) -> Result<ObjectId, Box<dyn Error>> {
        let serialized_client: Bson = bson::to_bson(&client)?;
        let document: &Document = serialized_client.as_document().unwrap();
        let insert_result = self.coll.insert_one(document, None).await?;
        let client_id: ObjectId = insert_result.inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        
        Ok(client_id)
    }

    #[allow(dead_code)]
    pub async fn db_find_client(&self, filter: &bson::Document) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    pub async fn db_delete_client(&self, id: &ObjectId) -> Result<(), Box<dyn Error>> {
        self.coll.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }
}
