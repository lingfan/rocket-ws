#![feature(proc_macro_hygiene, decl_macro)]

extern crate env_logger;
#[macro_use]
extern crate rocket;

use std::sync::mpsc::channel;
use std::thread;

use rocket_contrib::templates::Template;
/// Simple WebSocket server with error handling. It is not necessary to setup logging, but doing
/// so will allow you to see more details about the connection by using the RUST_LOG env variable.


use ws::listen;

use event::Event;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn wsserver() {
    // Listen on an address and call the closure for each connection
    if let Err(error) = listen("127.0.0.1:3012", |out| {
        // The handler needs to take ownership of out, so we use move
        move |msg| {
            // Handle messages received on this connection
            println!("Server got message '{}'. ", msg);

            // Use the out channel to send messages back
            out.send(msg)
        }
    }) {
        // Inform the user of failure
        println!("Failed to create WebSocket due to {:?}", error);
    }
}

fn main() {
    thread::spawn(move || wsserver());


    let (tx, rx) = channel::<Event>();
    let (tx_logging, rx_logging) = channel::<Event>();


    thread::spawn(|| ws::listen("127.0.0.1:3001", |out, rx| ws_server::server::Server::new(out, rx)));

    rocket::ignite()
        .mount("/", routes![index])
        .mount("/hello", routes![hello])
        .mount(
            "/api/v1/user",
            routes![api_user::user_id, api_user::logout, api_user::hello],
        )
        .mount("/api/cookie", routes![
            user::cookie::index,
            user::cookie::submit,
        ])
        .attach(Template::fairing())
        .launch();
}
