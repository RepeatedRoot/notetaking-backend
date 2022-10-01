use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
  pub user_id: ObjectId,
  pub password_hash: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginInfo {
  pub username: String,
  #[validate(length(min=8))]
  pub password: String
}