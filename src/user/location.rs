use std::io::Read;

use rocket::{Data, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};

use crate::mongodb::db::ThreadedDatabase;
use crate::user::User;
use crate::utils::mongo::connect_mongodb;

#[post("/time/<_time>")]
pub fn set_time(_time: String, _user: User) -> Status {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let document = doc! {
        "username" => _user.username
    };
    let new_doc = doc! {
        "$set" : {
            "time" => _time
        }
    };
    collection.update_one(document, new_doc, None).unwrap();
    return Status::Ok;
}

#[post("/country_code/<_country_code>")]
pub fn set_country_code(_country_code: String, _user: User) -> Status {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");
    let document = doc! {
        "username" => _user.username
    };
    let new_doc = doc! {
        "$set" : {
            "country_code" => _country_code
        }
    };
    collection.update_one(document, new_doc, None).unwrap();
    return Status::Ok;
}

pub struct Picture {
    pub picture: String
}

impl FromDataSimple for Picture {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Picture, String> {
        let mut string = String::new();

        //Read data
        if let Err(e) = data.open().take(32768).read_to_string(&mut string) {
            return Failure((Status::InternalServerError, format!("{:?}", e)));
        }

        //Parse data into json
        let json_res = match json::parse(&string) {
            Ok(t) => t,
            Err(_e) => return Failure((Status::UnprocessableEntity, ":".into()))
        };

        //Check for all fields in json
        let picture = json_res["picture"].to_string();
        if picture == "null" {
            return Failure((Status::UnprocessableEntity, ":".into()))
        }
        //return Success with user
        Success(Picture { picture })
    }
}

#[post("/picture", data = "<_picture>")]
pub fn picture(_picture: Picture, _user: User) -> Status {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
            "username" => _user.username,
    };
    let new_doc = doc! {
        "$set": {
            "picture": _picture.picture.clone()
        }
    };
    collection.update_one(document, new_doc, None).unwrap();
    Status::Ok
}
