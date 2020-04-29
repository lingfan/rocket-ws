use std;
use std::fs::File;
use std::io::Read;
use std::rc::Rc;
use std::sync::mpsc::Sender as ThreadSender;

use ws::{Builder, Sender, Settings};


use crate::event::Event;
use crate::settings::auth::Authorization;

mod server;
pub mod multicast;

pub fn run_server(connect_str: &str, max_connections: usize, tx: ThreadSender<Event>, auth: Authorization) {
    Builder::new().with_settings(Settings {
        max_connections: max_connections,
        panic_on_internal: false,
        encrypt_server: match ssl {
            Some(_) => true,
            _ => false
        },
        ..Settings::default()
    }).build(|out: Sender| {
        server::Server::new(out, tx.clone(), auth.clone())
    }).unwrap().listen(connect_str).unwrap();
}

fn read_file(name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(name)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    Ok(buf)
}
