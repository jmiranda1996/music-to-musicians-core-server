use crate::{models::user::User, repository::user::UserRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/user", data = "<data>")]
pub fn createUser(db: &State<UserRepo>, data: Json<User>) -> Result<Json<InsertOneResult>, Status> {
    let newUser = User {
        id: None,
        username: data.username.to_owned(),
        firstname: data.firstname.to_owned(),
        secondname: data.secondname.to_owned(),
        role: data.role.to_owned(),
        age: data.age,
    };
    let user = db.createUser(newUser);
    match user {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/user")]
pub fn getAllUsers(db: &State<UserRepo>) -> Result<Json<Vec<User>>, Status> {
    let users = db.getAllUsers();
    match users {
        Ok(users) => Ok(Json(users)),
        Err(_) => Err(Status::InternalServerError),
    }
}