extern crate chrono;

use chrono::Utc;
use jsonwebtoken::{decode, encode, Header, Validation};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use rocket::http::Status;

use crate::mongodb::db::ThreadedDatabase;
use crate::user::{get_user, get_username_with_token, User};
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
        Err(_e) => return Err(Status::Unauthorized),
    };
}

pub fn change_user_refresh_token(_refresh_token: String) -> (String, String) {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let _rng = thread_rng();
    let username = get_username_with_token(_refresh_token.clone());


    let document = doc! {
        "refresh_token" => _refresh_token.clone()
    };

    let token_refreshed: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();

    let updt = doc! {
        "$set": {
            "refresh_token": token_refreshed.clone()
        }
    };
    collection.update_one(document, updt, None).unwrap();

    let access_token = create_token(username.clone());
    (token_refreshed, access_token)
}

pub fn generate_first_refresh_token(_username: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let _rng = rand::thread_rng();

    let document = doc! {
        "username" => _username
    };

    let token_refreshed: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    let updt = doc! {
        "$set": {
            "refresh_token": token_refreshed.clone()
        }
    };
    collection.update_one(document, updt, None).unwrap();
    token_refreshed
}
