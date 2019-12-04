use rocket::http::Status;
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