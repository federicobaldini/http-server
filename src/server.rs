use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, error: &ParseError) -> Response {
    println!("Failed to parse a request: {}", error);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self { address: address }
  }
  pub fn run(self, mut handler: impl Handler) {
    println!("Listening on {}", self.address);

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 2048];
          match stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(error) => handler.handle_bad_request(&error),
              };
              if let Err(e) = response.send(&mut stream) {
                println!("Failed to send a response: {}", e);
              }
            }
            Err(error) => {
              println!("Failed to read from connection: {}", error);
            }
          }
        }
        Err(error) => {
          println!("Failed to establish a connection: {}", error);
        }
      }
    }
  }
}
