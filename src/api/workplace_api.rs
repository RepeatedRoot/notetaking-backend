use crate::{models::workplace_model::Workplace, repository::mongodb_repo::MongoRepo};
use rocket::{http::Status, serde::json::Json, State};

#[get("/workplace/<path>")]
pub fn get_workplace(db: &State<MongoRepo>, path: String) -> Result<Json<Workplace>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    }
    let workplace_detail = db.get_workplace(&id);

    match workplace_detail {
        Ok(workplace) => Ok(Json(workplace)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/workplaces")]
pub fn get_all_workplaces(db: &State<MongoRepo>) -> Result<Json<Vec<Workplace>>, Status> {
    let workplaces = db.get_all_workplaces();
    match workplaces {
        Ok(workplaces) => Ok(Json(workplaces)),
        Err(_) => Err(Status::InternalServerError),
    }
}
