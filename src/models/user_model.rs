use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Qualification {
  MedicalOfficer,
  ClinicalNurse,
  RegisteredNurse,
  PhysioTherapist,
  FamilySupportOfficer,
  ClientServicesOfficer
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub firstname: String,
  pub lastname: String,
  pub phone: String,
  pub workplace: ObjectId,
  pub qualification: Qualification
}