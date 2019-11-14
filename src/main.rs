#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#![feature(concat_idents)]

#[macro_use] extern crate rocket;

use rocket::Rocket;
/* -------------------- MODULES USAGES -------------------- */
mod user;

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![api_root])
        .mount("/user", routes![user::login::login, user::register::register])
}

fn main() {
    rocket().launch();
}

#[get("/")]
fn api_root() -> &'static str {
    "Welcome to GeoWorker API (tips: you shouldn't be here)"
}

/* -------------------- UNIT TESTS -------------------- */

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_root() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome to GeoWorker API (tips: you shouldn't be here)".into()));
    }
}