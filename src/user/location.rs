use std::io::Read;

use rocket::{Data, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::response::content;

use crate::mongodb::db::ThreadedDatabase;
use crate::user::User;
use crate::utils::mongo::connect_mongodb;

#[post("/location/<_country_code>")]
pub fn set_location(_country_code: String, _user: User) -> Status {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    println!("{} = {}", _user.username.clone(), _country_code.clone());

    let document = doc! {
        "username" => _user.username
    };
    let new_doc = doc! {
        "$set" : {
            "location" => _country_code
        }
    };
    collection.update_one(document, new_doc, None);
    return Status::Ok;
}

#[get("/location")]
pub fn location(_user: User) -> content::Json<String> {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
            "username" => _user.username,
    };
    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    let location = cursor.get("location").unwrap().as_str().unwrap();
    let res = content::Json(format!("{}\
        \"location\": \"{}\"\
        {}", "{", location, "}"));
    return res;
}

pub struct Picture {
    pub picture: String
}

impl FromDataSimple for Picture {
    type Error = String;

    fn from_data(_: &Request, data: Data) -> data::Outcome<Picture, String> {
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
    let cursor = collection.update_one(document, new_doc, None);
    Status::Ok
}