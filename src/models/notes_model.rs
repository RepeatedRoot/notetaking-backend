use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
  #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
  pub datetime: chrono::DateTime<Utc>,
  pub clinician: ObjectId,
  pub note: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteCollection {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>,
  pub notes: Vec<Note>
}