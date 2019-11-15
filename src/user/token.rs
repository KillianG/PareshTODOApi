extern crate chrono;

use chrono::Utc;
use jsonwebtoken::{decode, encode, Header, Validation};
use rand::Rng;
use rocket::http::Status;

use crate::mongodb::db::ThreadedDatabase;
use crate::user::{get_user, User};
use crate::utils::mongo::connect_mongodb;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
struct Claims {
    exp: i64,
}

pub fn create_token(_username: String) -> String {
    let my_claims = Claims {
        exp: Utc::now().timestamp() + 3600,
    };
    let mut header = Header::default();
    header.kid = Some(_username.clone());
    let token = encode(&header, &my_claims, "secret_salty_encoded_string".as_ref()).unwrap();
    token
}

pub fn decode_token(_token: String) -> Result<User, Status> {
    match decode::<Claims>(&_token, "secret_salty_encoded_string".as_ref(), &Validation::default()) {
        Ok(_t) => return Ok(get_user(_t.header.kid.unwrap().to_string())),
        Err(_e) => return Err(Status::Forbidden),
    };
}

pub fn change_user_refresh_token(_refresh_token: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let mut rng = rand::thread_rng();

    let document = doc! {
        "refresh_token" => _refresh_token
    };

    let token_refreshed = rng.gen_ascii_chars().take(30).collect::<String>();
    let updt = doc! {
        "$set": {
            "refresh_token": token_refreshed.clone()
        }
    };
    collection.update_one(document, updt, None).unwrap();
    token_refreshed
}

pub fn generate_first_refresh_token(_username: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let mut rng = rand::thread_rng();

    let document = doc! {
        "username" => _username
    };

    let token_refreshed = rng.gen_ascii_chars().take(30).collect::<String>();
    let updt = doc! {
        "$set": {
            "refresh_token": token_refreshed.clone()
        }
    };
    collection.update_one(document, updt, None).unwrap();
    token_refreshed
}