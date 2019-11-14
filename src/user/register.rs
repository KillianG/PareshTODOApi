use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::response::status;
use std::io::Read;

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

#[post("/register", data = "<_user>")]
pub fn register(_user: User) -> status::Created<String> {
    //add database connection and add user in
    status::Created(format!("User {} created!", _user.username), Some(format!("User {} created!", _user.username)))
}

/* -------------------- UNIT TESTS -------------------- */

#[cfg(test)]
mod test {
    use super::super::super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_register_ok() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/register").body(r#"{ "username": "tester", "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::Created);
        assert_eq!(response.body_string(), Some("User tester created!".into()));
    }

    #[test]
    fn test_register_no_username() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/register").body(r#"{ "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_no_password() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/register").body(r#"{ "username": "tester"}"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_no_args() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/register").dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }

    #[test]
    fn test_register_invalid_json() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/register").body(r#" "userna "teste "password": "G00DP}"#).dispatch();
        assert_eq!(response.status(), Status::UnprocessableEntity);
    }
}