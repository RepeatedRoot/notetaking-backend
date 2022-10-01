use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

//Possible types of clinicians
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Qualification {
  MedicalOfficer,
  ClinicalNurse,
  RegisteredNurse,
  PhysioTherapist,
  FamilySupportOfficer,
  ClientServicesOfficer
}

/* To store information about a clinican */
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[validate(length(min=1))] //To ensure that this field is not blank
  pub firstname: String,
  #[validate(length(min=1))] //To ensuer that this field is not blank
  pub lastname: String,
  #[validate(phone)] //Ensure that the phone number is of valid format
  pub phone: String,
  pub workplace: ObjectId, //The ID of the workplace the clinican works at
  pub qualification: Qualification //The qualification of the clinician as described by the above
                                   //enum
}
