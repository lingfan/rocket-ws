use rocket::http::{Cookie, Cookies};
use rocket::http::RawStr;
use rocket::response::{Flash, Redirect};
use rocket_contrib::json::Json;

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
pub struct UpdateUserPayload {
    x: i64,
    y: i64,
}

#[post("/user_get")]
pub fn get() -> &'static str {
    "Hello, world!"
}

#[post("/user_set", data = "<payload>")]
pub fn set(payload: Json<UpdateUserPayload>) -> String {
   
    format!("Hello, {}!", payload.x)
}
