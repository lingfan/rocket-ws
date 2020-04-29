use rocket::http::{Cookie, Cookies};
use rocket::http::RawStr;
use rocket::response::{self, Flash, Redirect};
use rocket_contrib::json::Json;
use crate::user;

/// Retrieve the user's ID, if any.
#[get("/user_id")]
pub fn user_id(mut cookies: Cookies) -> Option<String> {
    cookies
        .get_private("user_id")
        .map(|cookie| format!("User ID: {}", cookie.value()))
}

/// Remove the `user_id` cookie.
#[post("/logout")]
pub fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to("/"), "Successfully logged out.")
}

#[get("/hello/<name>")]
pub fn hello(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[derive(Debug, Deserialize)]
struct GetInfoPayload {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUserPayload {
    x: i64,
    y: i64,
}

#[post("/user_get", data = "<payload>")]
pub fn get(payload: Json<GetInfoPayload>) -> Result<String, response::Failure> {
    match user::get(&payload.name) {
        Ok(()) => Ok("ok".parse().unwrap()),
        Err(e) => None
    }
}

#[post("/user_set", data = "<payload>")]
pub fn set(payload: Json<UpdateUserPayload>) -> Result<(), response::Failure> {
    let id = 1;
    match user::set(id, payload.x, payload.y) {
        Ok(_) => Ok(()),
        Err(_) => Err(response::Failure::from(Status::raw(400))),
    }
}
