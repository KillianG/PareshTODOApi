#[post("/login")]
pub fn login() -> &'static str {
    "Welcome to user login page"
}

/* -------------------- UNIT TESTS -------------------- */

#[cfg(test)]
mod test {
    use super::super::super::rocket;
    use rocket::local::Client;
    use rocket::http::Status;

    #[test]
    fn test_root() {
        let client = Client::new(rocket()).expect("valid src instance");
        let mut response = client.post("/user/login").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Welcome to user login page".into()));
    }
}