use mongodb::bson::oid::ObjectId;
use mongodb::bson::doc;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
  pub user_id: ObjectId,
  pub password_hash: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginInfo {
  username: String,
  password: String
}