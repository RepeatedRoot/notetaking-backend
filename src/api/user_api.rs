use crate::{models::user_model::User, repository::mongodb_repo::MongoRepo};
use mongodb::{bson::oid::ObjectId, results::InsertOneResult};
use rocket::{http::Status, serde::json::Json, State};
use rocket_validation::Validated;
use rocket::http::CookieJar;

/* Create a new user in the database */
#[post("/user", data = "<new_user>")]
pub fn create_user(db: &State<MongoRepo>, cookies: &CookieJar<'_>, new_user: Validated<Json<User>>) -> Result<Json<InsertOneResult>, Status> {
  let authorised = db.check_auth(cookies);
  
  if authorised {
    let new_user = new_user.into_inner().into_inner();
    
    /* Parse JSON into User struct */
    let data = User {
      id: None,
      firstname: new_user.firstname.to_owned(),
      lastname: new_user.lastname.to_owned(),
      phone: new_user.phone.to_owned(),
      email: new_user.email.to_owned(),
      workplace: new_user.workplace.to_owned(),
      qualification: new_user.qualification.clone(),
      password: new_user.password.to_owned()
    };
  
    let user_detail = db.create_user(data); //create a new user in the database using the new information
  
    /* Return the ID of the inserted User information, or an error if encountered */
    match user_detail {
      Ok(user) => Ok(Json(user)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
}

/* Get a user's information given their ID */
#[get("/user/<path>")]
pub fn get_user(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String) -> Result<Json<User>, Status> {
  let authorised = db.check_auth(cookies);
  
  if authorised {
    let id = path; //The ID of the user
    if id.is_empty() { //If no ID was sent
      return Err(Status::BadRequest); //return an error
    }
    
    let user_detail = db.get_user(&id); //Query the database for the user's information
  
    /* Return the user's information or an error, if encountered */
    match user_detail {
      Ok(user) => Ok(Json(user)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
}

/* Update a user's information given their ID */
#[put("/user/<path>", data="<new_user>")]
pub fn update_user(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String, new_user: Validated<Json<User>>) -> Result<Json<User>, Status> {
  let authorised = db.check_auth(cookies);
  
  if authorised {
    let new_user = new_user.into_inner(); //unwrap the new_user Structure from the validation wrapping
    
    let id = path;
    if id.is_empty() {
      return Err(Status::BadRequest);
    }
  
    let data = User {
      id: Some(ObjectId::parse_str(&id).unwrap()),
      firstname: new_user.firstname.to_owned(),
      lastname: new_user.lastname.to_owned(),
      phone: new_user.phone.to_owned(),
      email: new_user.email.to_owned(),
      workplace: new_user.workplace.to_owned(),
      qualification: new_user.qualification.clone(),
      password: None
    };
  
    let update_result = db.update_user(&id, data);
  
    match update_result {
      Ok(update) => {
        if update.matched_count == 1 {
          let updated_user_info = db.get_user(&id);
          match updated_user_info {
            Ok(user) => Ok(Json(user)),
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

/* Delete a user entry in the database given their ID */
#[delete("/user/<path>")]
pub fn delete_user(db: &State<MongoRepo>, cookies: &CookieJar<'_>, path: String) -> Result<Json<String>, Status> {
  let authorised = db.check_auth(cookies);

  if authorised {
    let id = path;
    if id.is_empty() {
      return Err(Status::BadRequest);
    }
  
    let result = db.delete_user(&id);
  
    match result {
      Ok(res) => {
        if res.deleted_count == 1 {
          Ok(Json("User successfully deleted".to_string()))
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

/* Get a list of all users in the database */
#[get("/users")]
pub fn get_all_users(db: &State<MongoRepo>, cookies: &CookieJar<'_>) -> Result<Json<Vec<User>>, Status> {
  let authorised = db.check_auth(cookies);

  if authorised {
    let users = db.get_all_users();
      
    match users {
      Ok(users) => Ok(Json(users)),
      Err(_) => Err(Status::InternalServerError)
    }
  } else {
    Err(Status::Forbidden)
  }
  
}