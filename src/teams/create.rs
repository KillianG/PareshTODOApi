use std::borrow::Borrow;

use rocket::http::Status;

use crate::mongodb::db::ThreadedDatabase;
use crate::utils::mongo::connect_mongodb;

fn add_team_to_db(_team: super::team::Team, _admin: &super::super::user::User) -> () {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    collection.insert_one(doc! {
        "name": _team.name,
        "logo": _team.logo,
        "administrator": _admin.username.clone(),
        "members": {
            "1": _admin.username.clone()
        }
    }, None).unwrap();
}

#[post("/create", data = "<_team>")]
pub fn create(_team: super::team::Team, _user: super::super::user::User) -> Status {
    add_team_to_db(_team, _user.borrow());
    super::team::find_team(_user);
    Status::Ok
}