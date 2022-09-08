use crate::{models::notes_model::{Note, NoteCollection}, repository::mongodb_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};
use chrono;

#[get("/notes")]
pub fn create_notes(db: &State<MongoRepo>) -> Result<Json<InsertOneResult>, Status> {
  let note_collection_detail = db.create_notes();

  match note_collection_detail {
    Ok(note_collection) => Ok(Json(note_collection)),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[get("/notes/<path>")]
pub fn get_notes(db: &State<MongoRepo>, path: String) -> Result<Json<NoteCollection>, Status> {
  let id = path;
  if id.is_empty() {
    return Err(Status::BadRequest);
  }

  let notes_detail = db.get_notes(&id);

  match notes_detail {
    Ok(notes) => Ok(Json(notes)),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[put("/notes/<path>", data="<new_note>")]
pub fn add_note(db: &State<MongoRepo>, path: String, new_note: Json<Note>) -> Result<Json<NoteCollection>, Status> {
  let id = path;
  if id.is_empty() {
    return Err(Status::BadRequest);
  }

  let data = Note {
    datetime: new_note.datetime.to_owned(),
    clinician: new_note.clinician,
    note: new_note.note.to_owned()
  };

  let update_result = db.add_note(&id, data);

  match update_result {
    Ok(update) => {
      if update.matched_count == 1 {
        let updated_notes_collection = db.get_notes(&id);
        match updated_notes_collection {
          Ok(notes) => Ok(Json(notes)),
          Err(_) => Err(Status::InternalServerError)
        }
      } else {
        Err(Status::NotFound)
      }
    },
    Err(_) => Err(Status::InternalServerError)
  }
}
