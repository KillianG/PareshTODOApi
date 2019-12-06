use std::io::Read;

use rocket::{Data, Request};
use rocket::data::{self, FromDataSimple};
use rocket::http::Status;
use rocket::outcome::Outcome::{Failure, Success};

use crate::mongodb::db::ThreadedDatabase;
use crate::user::{get_user_extended, UserExtended};
use crate::utils::mongo::connect_mongodb;

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub name: String,
    pub logo: String,
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
    collection.update_one(document, upd, None).unwrap();
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
    collection.update_one(document, upd, None).unwrap();
}

pub fn remove_user_from_team(_team_id: String, _username: String) -> () {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap(),
    };
    let upd = doc! {
        "$pull": {
            "members": _username.clone()
        }
    };
    collection.update_one(document, upd, None).unwrap();
}

pub fn remove_team_from_user(_team_id: String, _username: String) -> () {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
            "username" => _username.clone()
    };
    let upd = doc! {
        "$pull": {
            "teams": _team_id.clone()
        }
    };
    collection.update_one(document, upd, None).unwrap();
}

pub fn remove_admin(_team_id: String) -> () {
    let members = get_members(_team_id.clone());
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");
    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap()
    };
    if members.len() > 0 {
        let upd = doc! {
        "$set": {
            "administrator": members.get(0).unwrap().username.clone()
            }
        };
        collection.update_one(document, upd, None).unwrap();
    } else {
        collection.find_one_and_delete(document, None).unwrap();
    }
}

pub fn get_team_by_id(_id: String) -> Team {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_id.as_ref()).unwrap(),
    };
    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    let name = cursor.get("name").unwrap().as_str().unwrap();
    let logo = cursor.get("logo").unwrap().as_str().unwrap();
    return Team { name: name.to_string(), logo: logo.to_string() };
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

pub fn is_member(_username: String, _team_id: String) -> bool {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap(),
    };
    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    let members = cursor.get("members").unwrap().as_array().unwrap();
    for member in members {
        if member.as_str().unwrap() == _username {
            return true;
        }
    };
    return false;
}

pub fn get_members(_team_id: String) -> Vec<UserExtended> {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let document = doc! {
            "_id" => mongodb::oid::ObjectId::with_string(_team_id.as_ref()).unwrap(),
    };
    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    let members = cursor.get("members").unwrap().as_array().unwrap();

    let mut res: Vec<UserExtended> = Vec::new();
    for member in members {
        let extended = get_user_extended(member.as_str().unwrap().to_string());
        res.push(extended)
    };
    return res;
}

pub fn find_user_teams(_username: String) -> std::vec::Vec<mongodb::Bson> {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("users");

    let document = doc! {
        "username" => _username.clone()
    };

    let cursor = collection.find_one(Some(document), None).unwrap().unwrap();
    cursor.get("teams").unwrap().as_array().unwrap().clone()
}

pub fn find_team_id(_username: String, _team_name: String) -> String {
    let db: std::sync::Arc<mongodb::db::DatabaseInner> = connect_mongodb();
    let collection = db.collection("teams");

    let user_teams = find_user_teams(_username.clone());

    for team in user_teams {
        let team_id = team.as_str().unwrap();

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

impl FromDataSimple for Team {
    type Error = String;

    fn from_data(_request: &Request<'r>, data: Data) -> data::Outcome<Team, String> {
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
            logo
        })
    }
}