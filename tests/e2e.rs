use rust_http_server::config::Config;
use rust_http_server::http::{content_type_for_path, Method, Request, RequestBody, Response, StatusCode};
use rust_http_server::server::{Handler, Server};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::OnceLock;
use std::thread;
use std::time::Duration;

const TEST_ADDR: &str = "127.0.0.1:7979";

struct TestHandler;

impl Handler for TestHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/hello" => {
          let mut response: Response =
            Response::new(StatusCode::Ok, Some("Hello, World!".to_string()));
          response.set_header(
            "Content-Type".to_string(),
            content_type_for_path("/hello.html").to_string(),
          );
          response
        }
        _ => Response::new(StatusCode::NotFound, None),
      },
      Method::POST => match request.path() {
        "/echo" => match request.body() {
          Some(RequestBody::Text(text)) => Response::new(StatusCode::Ok, Some(text.clone())),
          _ => Response::new(StatusCode::Ok, None),
        },
        _ => Response::new(StatusCode::MethodNotAllowed, None),
      },
      _ => Response::new(StatusCode::MethodNotAllowed, None),
    }
  }
}

static SERVER_STARTED: OnceLock<()> = OnceLock::new();

fn start_test_server() {
  SERVER_STARTED.get_or_init(|| {
    thread::spawn(|| {
      Server::new(Config {
        host: "127.0.0.1".to_string(),
        port: "7979".to_string(),
        ..Config::default()
      })
      .run(TestHandler);
    });
    // Give the server time to bind and start accepting connections
    thread::sleep(Duration::from_millis(100));
  });
}

fn send_raw_request(raw: &str) -> String {
  let mut stream: TcpStream = TcpStream::connect(TEST_ADDR).unwrap();
  stream
    .set_read_timeout(Some(Duration::from_secs(2)))
    .unwrap();
  stream.write_all(raw.as_bytes()).unwrap();
  stream.shutdown(std::net::Shutdown::Write).unwrap();
  let mut response: String = String::new();
  stream.read_to_string(&mut response).unwrap();
  response
}

#[test]
fn get_known_path_returns_200_with_body() {
  start_test_server();
  let resp: String = send_raw_request("GET /hello HTTP/1.1\r\nHost: localhost\r\n\r\n");
  assert!(resp.starts_with("HTTP/1.1 200 Ok"));
  assert!(resp.contains("Hello, World!"));
  assert!(resp.contains("Content-Type: text/html; charset=utf-8"));
  assert!(resp.contains("Content-Length:"));
}

#[test]
fn get_unknown_path_returns_404() {
  start_test_server();
  let resp: String = send_raw_request("GET /missing HTTP/1.1\r\nHost: localhost\r\n\r\n");
  assert!(resp.starts_with("HTTP/1.1 404 Not Found"));
}

#[test]
fn invalid_request_returns_400() {
  start_test_server();
  // Malformed request line: no path or protocol
  let resp: String = send_raw_request("NOTHTTP\r\n\r\n");
  assert!(resp.starts_with("HTTP/1.1 400 Bad Request"));
}

#[test]
fn non_get_method_returns_405() {
  start_test_server();
  let resp: String = send_raw_request("POST /hello HTTP/1.1\r\nHost: localhost\r\n\r\n");
  assert!(resp.starts_with("HTTP/1.1 405 Method Not Allowed"));
}

#[test]
fn post_with_text_body_is_echoed_back() {
  start_test_server();
  let resp: String = send_raw_request(
    "POST /echo HTTP/1.1\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhello",
  );
  assert!(resp.starts_with("HTTP/1.1 200 Ok"));
  assert!(resp.ends_with("hello"));
}

#[test]
fn request_larger_than_2048_bytes_is_handled() {
  start_test_server();
  // Header value of 3000 bytes makes the total request well over the old 2048-byte buffer limit
  let padding: String = "a".repeat(3000);
  let raw: String = format!(
    "GET /hello HTTP/1.1\r\nHost: localhost\r\nX-Padding: {}\r\n\r\n",
    padding
  );
  let resp: String = send_raw_request(&raw);
  assert!(resp.starts_with("HTTP/1.1 200 Ok"));
}
