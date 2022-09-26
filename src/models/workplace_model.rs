use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Workplace {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub name: String,
  pub address: String,
  #[validate(phone)]
  pub phone: String
}
