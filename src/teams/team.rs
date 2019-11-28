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

pub fn find_team(_user: super::super::user::User) {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let mut cursor = collection.find(None, None).unwrap();
    for result in cursor {
        if let Ok(item) = result {
            let a: serde_json::value::Value = item.get("members").unwrap().clone().into();
            println!("{}", a);
        }
    }
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