/* Include dependencies */
use crate::{models::auth_model::LoginInfo, repository::mongodb_repo::MongoRepo};
use mongodb::bson::doc;
use rocket::{http::Status, serde::json::Json, State};
use rocket::http::{Cookie, CookieJar};
use rocket_validation::Validated;

/* Login to the webpage */
#[post("/login", data="<login_info>")]
pub fn login(db: &State<MongoRepo>, cookies: &CookieJar<'_>, login_info: Validated<Json<LoginInfo>>) -> Result<(), Status> {
  let new_login = LoginInfo {
    username: login_info.0.username.to_owned(),
    password: login_info.0.password.to_owned()
  };

  let maybe_user = db.users.find_one(doc! { "email": new_login.username }, None);

  match maybe_user {
    Ok(Some(user)) => {
      let maybe_auth = db.auth.find_one(doc! { "user_id": user.id }, None);

      match maybe_auth {
        Ok(Some(auth_info)) => {
          let hash = MongoRepo::hash_password(&new_login.password);

          if hash == auth_info.password_hash {
            cookies.add_private(Cookie::new("user_id", user.id.expect("No User ID available").to_hex()));
            Ok(())
          } else {
            Err(Status::Forbidden)
          }
        },
        _ => Err(Status::Forbidden)
      }
    },
    _ => Err(Status::Forbidden)
  }
}

/* Logout from the website by removing the user_id private cookie */
#[post("/logout")]
pub fn logout(cookies: &CookieJar<'_>) -> () {
  cookies.remove_private(Cookie::named("user_id"));
}

/* Get the ID of the user by reading their private cookie */
#[get("/user_id")]
pub fn user_id(cookies: &CookieJar<'_>) -> Result<String, Status> {
  let id = cookies.get_private("user_id");

  match id {
    Some(id) => Ok(id.value().to_string()),
    _ => Err(Status::InternalServerError)
  }
}