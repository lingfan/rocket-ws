#![feature(decl_macro)]

extern crate bson;
extern crate chrono;
extern crate config;
extern crate crypto;
extern crate hex;
extern crate httparse;
#[macro_use]
extern crate log;
extern crate r2d2_redis;
extern crate redis;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate sidekiq;
extern crate url;
extern crate walkdir;
extern crate ws;

mod api_user;
mod user;
mod ws_server;
mod event;
mod utils;
mod settings;
mod notifier;