use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};

/* An individual note entry */
#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
  pub datetime: String,     //The time when the note was created
  pub clinician: ObjectId,  //The clinician who made the note
  pub note: String          //The note itself
}

/* A collection of notes */
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteCollection {
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>, //The ID of the collection of notes
  pub notes: Vec<Note>      //An array of notes structs
}
