use config::Config;
use server::Server;
use website_handler::WebsiteHandler;

mod config;
mod http;
mod server;
mod website_handler;

fn main() {
  let config: Config = Config::from_env();
  let public_path: String = config.public_path.clone();
  Server::new(config).run(WebsiteHandler::new(public_path));
}
