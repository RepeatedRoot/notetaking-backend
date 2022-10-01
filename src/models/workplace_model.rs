use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

/* Implementing traits for serialisation, deserialisation and validation */
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Workplace {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")] //The ID of the workplace,
                                                                    //renaming to fit database
                                                                    //convention
  pub id: Option<ObjectId>,
  pub name: String, //name of the workplace
  pub address: String, //the address of the workplace
  #[validate(phone)] //Validate the phone number to ensure it correct
  pub phone: String
}
