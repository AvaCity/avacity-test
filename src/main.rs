mod client;
mod inventory;
mod encoder;
mod decoder;
mod server;
mod common;
mod errors;
mod base_messages;
mod modules;
use server::Server;

fn main() {
    let srv = Server::new();
    srv.listen();
}
