use std::io::Read;

use rocket::{Data, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::response::content;

use crate::user::token;
use crate::user::token::change_user_refresh_token;

#[post("/login", data = "<_user>")]
pub fn login(_user: super::User) -> Result<content::Json<String>, Status> {
    if !super::user_exist(_user.username.clone()) {
        return Err(Status::BadRequest);
    }

    let hashed_password = super::hash_password(_user.password);
    let user_db = super::get_user(_user.username.clone());
    if user_db.password != hashed_password {
        return Err(Status::BadRequest);
    }
    let token = token::create_token(_user.username.clone());
    let refresh_token = token::generate_first_refresh_token(_user.username);
    return Ok(content::Json(format!("{}\
        'access_token': {},\
        'refresh_token': {}\
    {}", "{", token, refresh_token, "}")))
}

pub struct Token {
    token: String
}

impl FromDataSimple for Token {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Token, String> {
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
        let refresh_token = json_res["refresh_token"].to_string();
        if refresh_token == "null" {
            return Failure((Status::UnprocessableEntity, ":".into()))
        }
        //return Success with user
        Success(Token { token: refresh_token })
    }
}

#[post("/refresh_token", data = "<_token>")]
pub fn refresh_token(_token: Token) -> content::Json<String> {
    let (refresh_token, access_token) = change_user_refresh_token(_token.token);
    return content::Json(format!("{}\
        'refresh_token': {},\
        'access_token': {}
    {}", "{", refresh_token, access_token, "}"))
}

/* -------------------- UNIT TESTS -------------------- */

#[cfg(test)]
mod test {
    use rocket::http::Status;
    use rocket::local::Client;

    use super::super::super::rocket;

    #[test]
    fn test_login_ok() {
        let client = Client::new(rocket()).expect("valid src instance");
        let response = client.post("/user/login").body(r#"{ "username": "tester_static", "password": "G00DP4SSW0RD" }"#).dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}