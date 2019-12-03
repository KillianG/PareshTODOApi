use std::borrow::Borrow;
use std::io::Read;

use json::Array;
use rocket::{Data, Outcome, Request};
use rocket::data::FromDataSimple;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};

use crate::mongodb::db::ThreadedDatabase;
use crate::teams::team::is_admin;
use crate::user::User;
use crate::utils::mongo::connect_mongodb;

use super::team;

pub struct JoinPayload {
    team_name: String,
    invited_username: String,
}

impl FromDataSimple for JoinPayload {
    type Error = String;

    fn from_data(request: &Request<'r>, data: Data) -> Outcome<Self, (Status, Self::Error), Data> {
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

        let (team_name, invited_username) = (json_res["team_name"].to_string(), json_res["invited_username"].to_string());
        if team_name == "null" || invited_username == "null" {
            return Failure((Status::UnprocessableEntity, ":".into()));
        }

        Success(JoinPayload {
            team_name,
            invited_username,
        })
    }
}

fn find_user_teams(_username: String) -> std::vec::Vec<mongodb::Bson> {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username.clone()
    };

    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    cursor.get("teams").unwrap().as_array().unwrap().clone()
}

fn find_team_id(_username: String, _team_name: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let user_teams = find_user_teams(_username.clone());

    for team in user_teams {
        let team_id = team.as_str().unwrap();
        println!("{}", team_id);

        let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(team_id).unwrap(),
            "name" => _team_name.clone()
        };

        let cursor = collection.find_one(Some(document), None).unwrap();
        match cursor {
            Some(_t) => return team_id.parse().unwrap(),
            None => continue
        }
    }
    return "Error".to_string();
}

#[post("/join", data = "<_join_payload>")]
pub fn join(_join_payload: JoinPayload, _user: super::super::user::User) -> Status {
    let team_id = find_team_id(_user.username.clone(), _join_payload.team_name);
    if team::is_admin(_user.username.clone(), team_id.clone()) {
        println!("admin");
        team::add_team_to_user(team_id.clone(), _join_payload.invited_username.clone());
        team::add_user_to_team(_join_payload.invited_username.clone(), team_id);
        return Status::Ok;
    }
    println!("pas admin");
    Status::Forbidden
}