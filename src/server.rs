use std::net::TcpListener;

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
        Ok((stream, _)) => {
          /* manage the new connection */
        },
        Err(error) => {
          println!("Failed to establish a connection: {}", error);
        }
      }
    }
  }
}
