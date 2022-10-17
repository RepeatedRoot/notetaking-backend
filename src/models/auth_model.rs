use mongodb::bson::oid::ObjectId;
use rocket_validation::Validate;
use serde::{Deserialize, Serialize};

//Account entry in the database
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub user_id: ObjectId,     //The ID of the user
    pub password_hash: String, //The hash of the user's password
}

//Information used when logging in
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct LoginInfo {
    pub username: String, //Username
    #[validate(length(min = 8))]
    pub password: String, //Password (must be over 8 characters long)
}
