use std::io::Read;

use rocket::{Data, Outcome, Request};
use rocket::data::FromDataSimple;
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};

use crate::teams::team::find_team_id;

use super::team;

pub struct JoinPayload {
    team_name: String,
    invited_username: String,
}

impl FromDataSimple for JoinPayload {
    type Error = String;

    fn from_data(_request: &Request<'r>, data: Data) -> Outcome<Self, (Status, Self::Error), Data> {
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

#[post("/invite", data = "<_join_payload>")]
pub fn invite(_join_payload: JoinPayload, _user: super::super::user::User) -> Status {
    let team_id = find_team_id(_user.username.clone(), _join_payload.team_name);
    if team::is_admin(_user.username.clone(), team_id.clone()) {
        team::add_team_to_user(team_id.clone(), _join_payload.invited_username.clone());
        team::add_user_to_team(_join_payload.invited_username.clone(), team_id);
        return Status::Ok;
    }
    Status::Forbidden
}