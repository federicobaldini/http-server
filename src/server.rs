use crate::config::Config;
use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

// Size of each read chunk pulled from the TCP stream; internal tuning constant
const READ_CHUNK: usize = 4 * 1024;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;

  fn handle_bad_request(&mut self, error: &ParseError) -> Response {
    eprintln!("Failed to parse a request: {}", error);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  address: String,
  max_header_size: usize,
  max_body_size: usize,
  read_timeout: Duration,
}

impl Server {
  pub fn new(config: Config) -> Self {
    Self {
      address: format!("{}:{}", config.host, config.port),
      max_header_size: config.max_header_size,
      max_body_size: config.max_body_size,
      read_timeout: Duration::from_secs(config.read_timeout_secs),
    }
  }

  // Reads a complete HTTP request from the stream into a Vec<u8>.
  // Stops after headers + Content-Length body bytes have been received.
  // Returns an error if the header section exceeds max_header_size.
  // Body bytes beyond max_body_size are silently capped.
  fn read_request(&self, stream: &mut TcpStream) -> io::Result<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();
    let mut chunk: [u8; READ_CHUNK] = [0; READ_CHUNK];

    loop {
      let n: usize = stream.read(&mut chunk)?;
      if n == 0 {
        break;
      }
      buf.extend_from_slice(&chunk[..n]);

      // Reject oversized headers before the blank line appears
      if buf.len() > self.max_header_size && !buf.windows(4).any(|w| w == b"\r\n\r\n") {
        return Err(io::Error::new(
          io::ErrorKind::InvalidData,
          "request headers exceed size limit",
        ));
      }

      if let Some(header_end) = buf.windows(4).position(|w| w == b"\r\n\r\n") {
        // Extract Content-Length to know exactly how many body bytes to read
        let content_length: usize =
          Self::extract_content_length(&buf[..header_end]).min(self.max_body_size);
        let total_expected: usize = header_end + 4 + content_length;

        // Keep reading until the full body has arrived or the stream closes
        while buf.len() < total_expected {
          let n: usize = stream.read(&mut chunk)?;
          if n == 0 {
            break;
          }
          buf.extend_from_slice(&chunk[..n]);
        }

        buf.truncate(total_expected);
        return Ok(buf);
      }
    }

    Ok(buf)
  }

  // Scans raw header bytes for a Content-Length value; returns 0 if absent or unparseable
  fn extract_content_length(header_bytes: &[u8]) -> usize {
    let header_str: &str = std::str::from_utf8(header_bytes).unwrap_or("");
    for line in header_str.lines() {
      if line.to_ascii_lowercase().starts_with("content-length:") {
        if let Some(val) = line.splitn(2, ':').nth(1) {
          if let Ok(n) = val.trim().parse::<usize>() {
            return n;
          }
        }
      }
    }
    0
  }

  // The run method starts the server and begins handling incoming connections
  pub fn run(self, mut handler: impl Handler) {
    println!("Listening on {}", self.address);

    // Binds the TcpListener to the specified address
    let listener: TcpListener = TcpListener::bind(&self.address).unwrap_or_else(|error| {
      eprintln!("Failed to bind to {}: {}", self.address, error);
      std::process::exit(1);
    });

    // Continuously listens for incoming connections
    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          if let Err(error) = stream.set_read_timeout(Some(self.read_timeout)) {
            eprintln!("Failed to set read timeout: {}", error);
          }
          match self.read_request(&mut stream) {
            Ok(buf) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buf));
              let response: Response = match Request::try_from(buf.as_slice()) {
                Ok(request) => handler.handle_request(&request),
                Err(error) => handler.handle_bad_request(&error),
              };
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
