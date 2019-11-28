#![feature(proc_macro_hygiene, decl_macro)]
#![feature(plugin)]
#![feature(concat_idents)]
#![feature(in_band_lifetimes)]

#[macro_use]
extern crate mongodb;
extern crate rand;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;

use rocket::http::Method;
use rocket::Rocket;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, Cors, CorsOptions, Error};

/* -------------------- MODULES USAGES -------------------- */
mod user;
mod utils;
mod teams;

fn make_cors() -> Cors {


    CorsOptions { // 5.
        allowed_origins: AllowedOrigins::default(),
        allowed_methods: AllowedMethods::default(), // 1.
        allowed_headers: AllowedHeaders::default(),
        allow_credentials: true,
        ..Default::default()
    }
        .to_cors()
        .expect("error while building CORS")
}

fn rocket() -> Rocket {
    rocket::ignite()
        .mount("/", routes![api_root])
        .mount("/user", routes![user::login::login, user::register::register, user::login::refresh_token, user::login::is_logged])
        .mount("/team", routes![teams::create::create])
        .attach(make_cors())
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
    use rocket::http::Status;
    use rocket::local::Client;

    use super::rocket;

    #[test]
    fn test_root() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome to GeoWorker API (tips: you shouldn't be here)".into()));
    }
}