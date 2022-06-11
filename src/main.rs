mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::user::{createUser, getAllUsers};
use repository::user::UserRepo;

#[launch]
fn rocket() -> _ {
    let db = UserRepo::init();
    rocket::build()
        .manage(db)
        .mount("/", routes![createUser])
        .mount("/", routes![getAllUsers])
}