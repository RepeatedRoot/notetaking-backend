use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    {Client, Collection},
};
use chrono::{TimeZone, Utc};
use std::error::Error;

use crate::validation::{err_if_none, get_or_err};

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_names: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub dob: chrono::DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sex: Option<Sex>
}

#[derive(Debug, Clone)]
pub struct UserMgr {
    coll: Collection<User>
}

/* Functions for user management */
impl UserMgr {
    pub async fn new(db_url: &str) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::with_uri_str(db_url).await?;
        let coll = client.database(DB_NAME).collection(COLL_NAME);
        Ok(Self { coll } )
    }

    pub async fn db_insert_client(&self, client: &mut User) -> Result<ObjectId, Box<dyn Error>> {
        err_if_none(&client.surname, "surname")?;
        err_if_none(&client.given_names, "given_names")?;
        err_if_none(&client.sex, "sex")?;
        let insert_result = self.coll.insert_one(client, None).await?;
        let client_id: ObjectId = insert_result.inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        Ok(client_id)
    }

    pub async fn db_delete_client(&self, client: &mut User) -> Result<(), Box<dyn Error>> {
        let id = get_or_err(client.id.as_ref(), "id")?;
        self.coll.delete_one(doc! {"_id": id}, None).await?;
        Ok(())
    }
}
