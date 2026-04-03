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
  // The host and port are read from environment variables, with sensible defaults
  let host: String = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
  let port: String = env::var("PORT").unwrap_or_else(|_| "5000".to_string());
  let server: Server = Server::new(format!("{}:{}", host, port));
  server.run(WebsiteHandler::new(public_path));
}
