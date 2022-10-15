use crate::{models::notes_model::{Note, NoteCollection}, repository::mongodb_repo::MongoRepo};
use mongodb::{results::InsertOneResult, bson::oid::ObjectId};
use rocket::{http::Status, serde::json::Json, State};
use rocket::http::CookieJar;

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
    Ok(Some(notes)) => Ok(Json(notes)),
    Ok(None) => Err(Status::NoContent),
    Err(_) => Err(Status::InternalServerError)
  }
}

#[put("/notes/<path>", data="<new_note>")]
pub fn add_note(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String, new_note: Json<Note>) -> Result<Json<NoteCollection>, Status> {
  let authorised = db.check_auth(cookies);

  if authorised {
    let id = path;
    if id.is_empty() {
      return Err(Status::BadRequest);
    }

    let clinician_id = match cookies.get_private("user_id") {
      Some(id) => id.value().to_string(),
      _ => return Err(Status::Forbidden)
    };
    
    let data = Note {
      datetime: new_note.datetime.to_owned(),
      clinician: ObjectId::parse_str(clinician_id).unwrap(),
      note: new_note.note.to_owned()
    };
  
    let update_result = db.add_note(&id, data);
  
    match update_result {
      Ok(update) => {
        if update.matched_count == 1 {
          let updated_notes_collection = db.get_notes(&id);
          match updated_notes_collection {
            Ok(Some(notes)) => Ok(Json(notes)),
            Ok(None) => Err(Status::NoContent),
            Err(_) => Err(Status::InternalServerError)
          }
        } else {
          Err(Status::NotFound)
        }
      },
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
}
