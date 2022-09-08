use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sex {
  Male,
  Female
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CafhsClient {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub firstname: String,
  pub surname: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub middlenames: Option<String>,
  pub sex: Sex,
  pub address: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub postal_address: Option<String>,
  pub phone: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub connections: Option<Vec<ObjectId>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub notes: Option<ObjectId>
}
