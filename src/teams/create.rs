use std::borrow::Borrow;

use json::Array;
use rocket::http::Status;

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

use super::team;

#[post("/create", data = "<_team>")]
pub fn create(_team: super::team::Team, _user: super::super::user::User) -> Status {
    let team_id = team::add_team_to_db(_team.borrow(), _user.borrow());
    team::add_team_to_user(team_id, _user.username);
    Status::Ok
}