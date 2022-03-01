#![allow(dead_code)]

use server::Server;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
  let server = Server::new("127.0.0.1:5000".to_string());
  server.run(WebsiteHandler);
}
