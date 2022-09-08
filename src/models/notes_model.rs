use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
  pub datetime: String,
  pub clinician: ObjectId,
  pub note: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteCollection {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub notes: Vec<Note>
}
