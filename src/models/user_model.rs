use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use rocket_validation::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Qualification {
  MedicalOfficer,
  ClinicalNurse,
  RegisteredNurse,
  PhysioTherapist,
  FamilySupportOfficer,
  ClientServicesOfficer
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  #[validate(length(min=1))]
  pub firstname: String,
  #[validate(length(min=1))]
  pub lastname: String,
  #[validate(phone)]
  pub phone: String,
  pub workplace: ObjectId,
  pub qualification: Qualification
}
