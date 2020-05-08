use log::info;

use actix_files::Files;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use std::{
    sync::{Arc, Mutex},
};

#[macro_use]
pub extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod event;
mod message;
mod server;
mod session;

use session::WsChatSession;
use event::EventEmitter;

async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WsChatSession::default(), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let event_emitter =EventEmitter::new();

    env_logger::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let addr = "127.0.0.1:8080";

    let srv = HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws/").to(chat_route))
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(&addr)?;

    info!("Starting http server: {}", &addr);

    srv.run().await
}
