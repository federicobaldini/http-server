use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, error: &ParseError) -> Response {
    eprintln!("Failed to parse a request: {}", error);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self { address }
  }

  // The run method starts the server and begins handling incoming connections
  pub fn run(self, mut handler: impl Handler) {
    println!("Listening on {}", self.address);

    // Binds the TcpListener to the specified address
    let listener: TcpListener = TcpListener::bind(&self.address).unwrap_or_else(|error| {
      eprintln!("Failed to bind to {}: {}", self.address, error);
      std::process::exit(1);
    });

    // Maximum number of bytes read per request; larger requests will be truncated
    const BUFFER_SIZE: usize = 2048;

    // Continuously listens for incoming connections
    loop {
      match listener.accept() {
        // If a connection is successfully established
        Ok((mut stream, _)) => {
          // Buffer to hold incoming data
          let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
          match stream.read(&mut buffer) {
            // If data is successfully read from the connection
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              // Attempts to convert the received data into a Request struct
              let response: Response = match Request::try_from(&buffer[..]) {
                Ok(request) => handler.handle_request(&request),
                Err(error) => handler.handle_bad_request(&error),
              };
              // Attempts to send the response back to the client
              if let Err(error) = response.send(&mut stream) {
                eprintln!("Failed to send a response: {}", error);
              }
            }
            Err(error) => {
              eprintln!("Failed to read from connection: {}", error);
            }
          }
        }
        Err(error) => {
          eprintln!("Failed to establish a connection: {}", error);
        }
      }
    }
  }
}
