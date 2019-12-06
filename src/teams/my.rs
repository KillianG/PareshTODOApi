use rocket::http::Status;
use rocket::response::content;

use crate::teams::team::{get_members, is_admin, Team};
use crate::user::User;

use super::team;

#[get("/my")]
pub fn my(_user: User) -> content::Json<String> {
    let teams = team::find_user_teams(_user.username);
    let mut team_vec: Vec<Team> = Vec::new();

    for team_tmp in teams {
        let team = team::get_team_by_id(team_tmp.as_str().unwrap().to_string());
        team_vec.push(team)
    }
    let serialized = serde_json::to_string(&team_vec).unwrap();
    return content::Json(serialized);
}

#[get("/members/<_team_name>")]
pub fn members(_team_name: String, _user: User) -> Result<content::Json<String>, Status> {
    let team_id = team::find_team_id(_user.username.clone(), _team_name);
    let is_member = team::is_member(_user.username.clone(), team_id.clone());
    if !is_member {
        return Err(Status::Forbidden);
    }
    let members = get_members(team_id.clone());
    let serialized = serde_json::to_string(&members).unwrap();
    return Ok(content::Json(serialized));
}

#[post("/leave/<_team_name>")]
pub fn leave(_team_name: String, _user: User) -> Status {
    let team_id = team::find_team_id(_user.username.clone(), _team_name);
    let is_member = team::is_member(_user.username.clone(), team_id.clone());
    if !is_member {
        return Status::Forbidden;
    }
    team::remove_team_from_user(team_id.clone(), _user.username.clone());
    team::remove_user_from_team(team_id.clone(), _user.username.clone());
    if is_admin(_user.username.clone(), team_id.clone()) {
        team::remove_admin(team_id.clone());
    }
    return Status::Ok;
}