use server::Server;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
  // The default path for the public files is set to the "public" directory within the current cargo manifest directory
  let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
  // The actual public path is determined by checking for the presence of an environment variable named "PUBLIC_PATH"
  // If it exists, it is used as the public path. If it does not exist, the default path defined above is used instead.
  let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
  println!("Public path: {}", public_path);
  // A new server instance is created and bound to the IP address "127.0.0.1" and port 5000
  let server = Server::new("127.0.0.1:5000".to_string());
  server.run(WebsiteHandler::new(public_path));
}
