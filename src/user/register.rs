use rocket::http::Status;
use rocket::response::content;

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

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

fn add_user_to_db(_user: super::User) -> bool {
    if super::user_exist(_user.username.clone()) {
        return false;
    }
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let hashed = super::hash_password(_user.password);

    collection.insert_one(doc! {
        "username": _user.username,
        "password": hashed,
        "refresh_token": "",
        "location": "FI",
        "picture": "",
        "teams": []

    }, None).unwrap();
    true
}

#[post("/register", data = "<_user>")]
pub fn register(_user: super::Usper) -> Status {
    if !add_user_to_db(_user) {
        return Status::Conflict;
    }
    return Status::Created;
}

#[get("/exist/<_username>")]
pub fn exist(_username: String, _user: super::User) -> &'static str {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
            "username" => _username,
    };
    let cursor = collection.find_one(Some(document), None).unwrap();
    match cursor {
        Some(T) => return "True",
        None => return "False"
    }
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