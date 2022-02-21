use std::net::TcpListener;
use std::io::Read;

pub struct Server {
  address: String,
}

impl Server {
  pub fn new(address: String) -> Self {
    Self { address: address }
  }
  pub fn run(self) {
    println!("Listening on {}", self.address);

    let listener = TcpListener::bind(&self.address).unwrap();

    loop {
      match listener.accept() {
        Ok((mut stream, _)) => {
          let mut buffer = [0; 1024];
          stream.read(&mut buffer);
        },
        Err(error) => {
          println!("Failed to establish a connection: {}", error);
        }
      }
    }
  }
}
