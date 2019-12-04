use rocket::response::content;

use crate::mongodb::db::ThreadedDatabase;
use crate::teams::team::Team;
use crate::user::User;

use super::team;

#[get("/my")]
pub fn my(_user: User) -> content::Json<String> {
    let mut teams = team::find_user_teams(_user.username);
    let mut team_vec: Vec<Team> = Vec::new();

    for team_tmp in teams {
        let team = team::get_team_by_id(team_tmp.as_str().unwrap().to_string());
        team_vec.push(team)
    }
    let serialized = serde_json::to_string(&team_vec).unwrap();
    return content::Json(serialized);
}