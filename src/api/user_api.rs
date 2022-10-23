use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::http::CookieJar;
use rocket::{http::Status, serde::json::Json, State};
use rocket_validation::Validated;

/* Create a new user in the database */
#[post("/user", data = "<new_user>")]
pub fn create_user(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    new_user: Validated<Json<User>>,
) -> Result<Json<InsertOneResult>, Status> {
    let authorised = db.check_auth(cookies); //Check authentication status of user

    if authorised {
        //Authorised
        let new_user = new_user.into_inner().into_inner(); //Unwrap User struct from json payload

        let user_detail = db.create_user(new_user); //create a new user in the database using the new information

        /* Return the ID of the inserted User information, or an error if encountered */
        match user_detail {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        //Not authorised
        Err(Status::Forbidden)
    }
}

/* Get a user's information given their ID */
#[get("/user/<path>")]
pub fn get_user(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
) -> Result<Json<User>, Status> {
    let authorised = db.check_auth(cookies); //Check authentication status of user

    if authorised {
        //Authorised for this action
        let id = path; //The ID of the user
        if id.is_empty() {
            //If no ID was sent
            return Err(Status::BadRequest); //return an error
        }

        let user_detail = db.get_user(&id); //Query the database for the user's information

        /* Return the user's information or an error, if encountered */
        match user_detail {
            Ok(user) => Ok(Json(user)),
            Err(_) => Err(Status::InternalServerError),
        }
    } else {
        //Not authorised
        Err(Status::Forbidden)
    }
}

/* Update a user's information given their ID */
#[put("/user/<path>", data = "<new_user>")]
pub fn update_user(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
    new_user: Validated<Json<User>>,
) -> Result<Json<User>, Status> {
    let authorised = db.check_auth(cookies); //Check authentication status of user

    if authorised {
        //Authorised
        let new_user = new_user.into_inner(); //unwrap the new_user Structure from the validation wrapping

        /* Make sure that an ID was passed to endpoint */
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest); //There was no ID, return an error
        }

        /* Deserialise JSON into a User object */
        let data = User {
            id: Some(ObjectId::parse_str(&id).unwrap()),
            firstname: new_user.firstname.to_owned(),
            lastname: new_user.lastname.to_owned(),
            phone: new_user.phone.to_owned(),
            email: new_user.email.to_owned(),
            workplace: new_user.workplace.to_owned(),
            qualification: new_user.qualification.clone(),
            password: None,
        };

        /* Run the update function */
        let update_result = db.update_user(&id, data);

        /* Ensuring that the data was inserted correctly */
        match update_result {
            Ok(update) => {
                //An update occurred
                if update.matched_count == 1 {
                    //One entry was updated
                    let updated_user_info = db.get_user(&id); //Get the updated entry
                    match updated_user_info {
                        Ok(user) => Ok(Json(user)), //An entry was returned, serialise as JSON and return
                        Err(_) => Err(Status::InternalServerError), //No entry was returned, return an error
                    }
                } else {
                    //No entry was updated, return an error
                    Err(Status::NotFound)
                }
            }
            Err(_) => Err(Status::InternalServerError), //An error occurred
        }
    } else {
        //Not authenticated for this action
        Err(Status::Forbidden)
    }
}

/* Delete a user entry in the database given their ID */
#[delete("/user/<path>")]
pub fn delete_user(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
    path: String,
) -> Result<Json<String>, Status> {
    let authorised = db.check_auth(cookies); //Check for authorisation

    if authorised {
        //The user is authorised
        /* Ensure an ID was passed to the endpoint */
        let id = path;
        if id.is_empty() {
            return Err(Status::BadRequest); //No id was passed, return an error
        }

        let result = db.delete_user(&id); //Delete the entry corresponding to the ID

        match result {
            Ok(res) => {
                //A delete operation occurred
                if res.deleted_count == 1 {
                    //An entry was successfully deleted
                    Ok(Json("User successfully deleted".to_string())) //Send a confirmation message
                } else {
                    //No entry was deleted
                    Err(Status::NotFound) //Return an error
                }
            }
            Err(_) => Err(Status::InternalServerError), //No operation occurred, return an error
        }
    } else {
        //No authenticated for this action
        Err(Status::Forbidden) //return an error
    }
}

/* Get a list of all users in the database */
#[get("/users")]
pub fn get_all_users(
    db: &State<MongoRepo>,
    cookies: &CookieJar<'_>,
) -> Result<Json<Vec<User>>, Status> {
    let authorised = db.check_auth(cookies); //Try to retrieve private cookie

    if authorised {
        //User is authorised
        let users = db.get_all_users(); //Get a list of all users

        match users {
            Ok(users) => Ok(Json(users)), //A result was returned, serialise into JSON and return
            Err(_) => Err(Status::InternalServerError), //Nothing was returned, return an error
        }
    } else {
        //Not authenticated for this action
        Err(Status::Forbidden) //Return an error
    }
}
