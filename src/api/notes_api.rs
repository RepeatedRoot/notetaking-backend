use crate::{
    models::notes_model::{Note, NoteCollection},
    repository::mongodb_repo::MongoRepo,
};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::http::CookieJar;
use rocket::{http::Status, serde::json::Json, State};

/* Create a new note collection */
#[get("/notes")]
pub fn create_notes(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
) -> Result<Json<InsertOneResult>, Status> {
    let authorised = db.check_auth(cookies); //Check authentication status

    if authorised { //Authorised
        let note_collection_detail = db.create_notes(); //Create a new note entry

        match note_collection_detail {
            Ok(note_collection) => Ok(Json(note_collection)), //An entry was created, return it's ID
            Err(_) => Err(Status::InternalServerError), //No entry was created, return an error
        }
    } else {    //Not authorised
        Err(Status::Forbidden) //Return an error
    }
}

/* Get a collection of notes */
#[get("/notes/<path>")]
pub fn get_notes(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
) -> Result<Json<NoteCollection>, Status> {
    let authorised = db.check_auth(cookies); //Check authorisation status

    if authorised { //Authorised
        /* Check if ID was passed to endpoint */
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest); //There was no ID, return error
        }

        let notes_detail = db.get_notes(&id); //Get note collection matching ID

        match notes_detail {
            Ok(Some(notes)) => Ok(Json(notes)), //A collection of notes was return, serialise as JSON and return
            Ok(None) => Err(Status::NoContent), //Operation was successfull, but there were no notes. Return an error
            Err(_) => Err(Status::InternalServerError), //Operation was not successful, return an error
        }
    } else { //Not authorised
        Err(Status::Forbidden) //Return an error
    }
}

/* Insert a note into a collection */
#[put("/notes/<path>", data = "<new_note>")]
pub fn add_note(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
    new_note: Json<Note>,
) -> Result<Json<NoteCollection>, Status> {
    let authorised = db.check_auth(cookies); //Check authorisation status

    if authorised { //Authorised
        /* Check if ID was passed to endpoint */
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest); //no ID was passed, return error
        }

        let clinician_id = match cookies.get_private("user_id") {
            Some(id) => id.value().to_string(), //Cookie exists, return clinician ID value
            _ => return Err(Status::Forbidden), //There is no cookie, return error (break from function)
        };

        //Serialise note information
        let data = Note {
            datetime: new_note.datetime.to_owned(),
            clinician: ObjectId::parse_str(clinician_id).unwrap(),
            note: new_note.note.to_owned(),
        };

        let update_result = db.add_note(&id, data); //Add note to collection

        match update_result {
            Ok(update) => { //Update operation was successful
                if update.matched_count == 1 { //One entry was updated
                    let updated_notes_collection = db.get_notes(&id); //Get updated notes entry
                    match updated_notes_collection {
                        Ok(Some(notes)) => Ok(Json(notes)), //Operation successful, notes entry retrieved. Serialise to JSON and return
                        Ok(None) => Err(Status::NoContent), //Operation successful, no notes entry retrieved. Return error
                        Err(_) => Err(Status::InternalServerError), //Operation unsuccessful, return error
                    }
                } else { //No entry was updated
                    Err(Status::NotFound) //Return entry
                }
            }
            Err(_) => Err(Status::InternalServerError), //Return error
        }
    } else { //Not authorised
        Err(Status::Forbidden) //Return error
    }
}
