use std::io::Read;

use rocket::{Data, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use sha2::{Digest, Sha256};

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

pub mod login;
pub mod register;
pub mod token;

pub struct User {
    username: String,
    password: String,
}

//Trait created for structure User
impl FromDataSimple for User {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<User, String> {
        let mut string = String::new();

        //Read data
        if let Err(e) = data.open().take(256).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        //Parse data into json
        let json_res = match json::parse(&string) {
            Ok(t) => t,
            Err(_e) => return Failure((Status::UnprocessableEntity, ":".into()))
        };

        //Check for all fields in json
        let (username, password) = (json_res["username"].to_string(), json_res["password"].to_string());
        if username == "null" || password == "null" {
            return Failure((Status::UnprocessableEntity, ":".into()))
        }
        //return Success with user
        Success(User { username, password })
    }
}

fn user_exist(_username: String) -> bool {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username
    };
    let cursor = collection.find(Some(document), None).unwrap();
    return cursor.count() != 0;
}

fn get_username_with_token(_token: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "refresh_token" => _token
    };
    let cursor = collection.find(Some(document), None).unwrap();
    for result in cursor {
        let doc = result.expect("Received network error during cursor operations.");
        return doc.get("username").unwrap().to_string().replace("\"", "");
    };
    "Error".to_string()
}

fn get_user(_username: String) -> User {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username
    };
    let cursor = collection.find(Some(document), None).unwrap();
    for result in cursor {
        let doc = result.expect("Received network error during cursor operations.");
        return User {
            username: doc.get("username").unwrap().to_string().replace("\"", ""),
            password: doc.get("password").unwrap().to_string().replace("\"", ""),
        };
    };
    return User { username: "ERROR".to_string(), password: "ERROR".to_string() }
}

fn hash_password(_password: String) -> String {
    // Hash user password
    let mut hasher = Sha256::new();
    hasher.input(_password.as_bytes());
    let result = hasher.result();

    let hashed = format!("{:x}", result);
    hashed
}