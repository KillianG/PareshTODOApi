use std::io::Read;

use rocket::{Data, Outcome::*, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use sha2::{Digest, Sha256};

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

pub struct User {
    username: String,
    password: String
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
        Success(User{ username, password })
    }
}

// Here I remove the warning for this 'unused' function because it is not unused
#[allow(dead_code)]
fn delete_user(_username: String) {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username
    };
    collection.find_one_and_delete(document, None).unwrap();
}

fn check_user_doesnt_exist(_username: String) -> bool {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username
    };
    let cursor = collection.find(Some(document), None).unwrap();
    return cursor.count() == 0;
}

fn add_user_to_db(_user: User) -> bool {
    if !check_user_doesnt_exist(_user.username.clone()) {
        return false;
    }
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    // Hash user password
    let mut hasher = Sha256::new();
    hasher.input(_user.password.as_bytes());
    let result = hasher.result();

    let hashed = format!("{:x}", result);

    collection.insert_one(doc! {
        "username": _user.username,
        "password": hashed
    }, None).unwrap();
    true
}

#[post("/register", data = "<_user>")]
pub fn register(_user: User) -> Status {
    if !add_user_to_db(_user) {
        return Status::Conflict;
    }
    return Status::Created;
}

/* -------------------- UNIT TESTS -------------------- */

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    use super::super::super::rocket;

    #[test]
    fn test_register_ok() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").body(r#"{ "username": "tester", "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::Created);
        super::delete_user("tester".to_string());
    }

    #[test]
    fn test_register_user_exist() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").body(r#"{ "username": "tester_static", "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::Conflict);
    }

    #[test]
    fn test_register_no_username() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").body(r#"{ "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_no_password() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").body(r#"{ "username": "tester"}"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_no_args() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_invalid_json() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/register").body(r#" "userna "teste "password": "G00DP}"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
}