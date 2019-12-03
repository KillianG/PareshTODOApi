use std::io::Read;

use json::Array;
use rocket::{Data, Outcome, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};
use rocket::request::FromRequest;

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

pub struct Team {
    pub name: String,
    pub logo: String,
    pub admin: String,
    pub members: Vec<String>,
}

pub fn add_user_to_team(_username: String, _team_id: String) -> () {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap(),
    };
    let upd = doc! {
        "$push": {
            "members": _username.clone()
        }
    };
    collection.update_one(document, upd, None);
}

pub fn add_team_to_db(_team: &super::team::Team, _admin: &super::super::user::User) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let res_id: mongodb::oid::ObjectId = collection.insert_one(doc! {
        "name": _team.name.clone(),
        "logo": _team.logo.clone(),
        "administrator": _admin.username.clone(),
        "members": [
            _admin.username.clone()
        ]
    }, None).unwrap().inserted_id.unwrap().as_object_id().unwrap().clone();
    res_id.to_hex()
}

pub fn add_team_to_user(_team_id: String, _username: String) -> () {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username.clone()
    };
    let upd = doc! {
        "$push": {
            "teams": _team_id.clone()
        }
    };
    collection.update_one(document, upd, None);
}

pub fn is_admin(_username: String, _team_id: String) -> bool {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap(),
    };
    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    return cursor.get("administrator").unwrap().as_str().unwrap() == _username
}

impl FromDataSimple for Team {
    type Error = String;

    fn from_data(request: &Request<'r>, data: Data) -> data::Outcome<Team, String> {
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

        let (name, logo) = (json_res["name"].to_string(), json_res["logo"].to_string());
        if name == "null" || logo == "null" {
            return Failure((Status::UnprocessableEntity, ":".into()));
        }

        Success(Team {
            name,
            logo,
            admin: "NOBODY".to_string(),
            members: vec![],
        })
    }
}