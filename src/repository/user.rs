use std::env;
    extern crate dotenv;
    use dotenv::dotenv;

    use mongodb::{
        bson::{extjson::de::Error},
        results::{ InsertOneResult},
        sync::{Client, Collection},
    };
    use crate::models::user::User;

    pub struct UserRepo {
        col: Collection<User>,
    }

    impl UserRepo {
        pub fn init() -> Self {
            dotenv().ok();
            let uri = match env::var("MONGO_URI") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variable"),
            };
            let client = Client::with_uri_str(uri).unwrap();
            let db = client.database("MusicToMusicians");
            let col: Collection<User> = db.collection("User");
            UserRepo { col }
        }

        pub fn getAllUsers(&self) -> Result<Vec<User>, Error> {
            let cursors = self
                .col
                .find(None, None)
                .ok()
                .expect("Error getting list of users");
            let users = cursors.map(|doc| doc.unwrap()).collect();
            Ok(users)
        }

        pub fn createUser(&self, newuser: User) -> Result<InsertOneResult, Error> {
            // let newDoc = User {
            //     id: None,
            //     username: newuser.username,
            //     firstname: newuser.firstname,
            //     secondname: newuser.secondname,
            //     role: newuser.role,
            //     age: newuser.age,
            // };
            let user = self
                .col
                .insert_one(newuser, None)
                .ok()
                .expect("Error creating user");
            Ok(user)
        }

    }