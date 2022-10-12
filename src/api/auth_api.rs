/* Include dependencies */
use crate::{models::auth_model::LoginInfo, repository::mongodb_repo::MongoRepo};
use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};
use rocket::http::{Cookie, CookieJar};
use rocket_validation::Validated;

/* Login to the webpage */
#[post("/login", data="<login_info>")]
pub fn login(db: &State<MongoRepo>, cookies: &CookieJar<'_>, login_info: Validated<Json<LoginInfo>>) -> Result<(), Status> {
  /* Login information entered by the user */
  let new_login = LoginInfo {
    username: login_info.0.username.to_owned(),
    password: login_info.0.password.to_owned()
  };

  /* Check if the user exists */
  let maybe_user = db.users.find_one(doc! { "email": new_login.username }, None);

  match maybe_user {
    Ok(Some(user)) => { // The user exists
      /* Check if the user has an associated entry for authentication */
      let maybe_auth = db.auth.find_one(doc! { "user_id": user.id }, None);

      match maybe_auth {
        Ok(Some(auth_info)) => { // The user has an auth entry
          let hash = MongoRepo::hash_password(&new_login.password); // Hash the entered password

          if hash == auth_info.password_hash { /* If the two hashes are the same */
            cookies.add_private(Cookie::new("user_id", user.id.expect("No User ID available").to_hex())); // Add the user's ID to private cookies
            Ok(()) //Everything was fine
          } else {
            Err(Status::Forbidden) // The password is incorrect
          }
        },
        _ => Err(Status::Forbidden) // There is not authentication entry
      }
    },
    _ => Err(Status::Forbidden) // There is no account for that username
  }
}

/* Logout from the website by removing the user_id private cookie */
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> () {
  cookies.remove_private(Cookie::named("user_id")); // Remove the user_id private cookie from the session
}

/* Get the ID of the user by reading their private cookie */
#[get("/user_id")]
pub fn user_id(cookies: &CookieJar<'_>) -> Result<String, Status> {
  let id = cookies.get_private("user_id"); // Get the value of the 'user_id' private cookie

  match id {
    Some(id) => Ok(id.value().to_string()), // The cookie exists, send it to the client webpage
    _ => Err(Status::InternalServerError)           // The cookie does not exist, return an error
  }
}