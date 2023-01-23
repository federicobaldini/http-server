use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::fs;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }

  // The read_file method reads a file from the specified path and returns its contents
  fn read_file(&self, file_path: &str) -> Option<String> {
    // Creates a path by combining the public_path field of the struct with the file_path parameter
    let path: String = format!("{}/{}", self.public_path, file_path);

    // Attempts to get the canonical path of the file
    match fs::canonicalize(path) {
      // If successful, check if the path starts with the public_path field of the struct
      Ok(path) => {
        // If the path is safe, read the contents of the file and return it
        if path.starts_with(&self.public_path) {
          fs::read_to_string(path).ok()
        } else {
          println!("Directory Traversal Attack Attempted: {}", file_path);
          None
        }
      }
      Err(_) => None,
    }
  }
}

// Only the GET method is handled
impl Handler for WebsiteHandler {
  // Handles incoming request
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
        "/test" => Response::new(
          StatusCode::Ok,
          Some("<h1>Path '/test' works!</h1>".to_string()),
        ),
        // For all other request paths, try to read the file at the requested path
        path => match self.read_file(path) {
          Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
          None => Response::new(StatusCode::NotFound, None),
        },
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}
