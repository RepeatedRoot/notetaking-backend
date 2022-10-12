use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

/* The possible genders of the client */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
  Male,
  Female
}

/* Information about a client */
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct CafhsClient {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,                         //The ID of the client
  #[validate(length(min=1))]                        //Ensure that this field is not blank
  pub firstname: String,
  #[validate(length(min=1))]                        //Ensure that this field is not blank
  pub surname: String,
  #[serde(skip_serializing_if = "Option::is_none")] //Optional field
  pub middlenames: Option<String>,
  pub sex: Sex,                                     //The gender of the client as described by the above enum
  pub address: String,                              //The address of the client
  #[serde(skip_serializing_if = "Option::is_none")] //Optional field
  pub postal_address: Option<String>,
  #[validate(phone)]                                //Ensure the phone number is of valid format
  pub phone: String,
  #[serde(skip_serializing_if = "Option::is_none")] //if the client is associated with other clients
  pub connections: Option<Vec<ObjectId>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notes: Option<ObjectId>                       //The ID of the client's notes
}
