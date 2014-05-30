use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};

pub struct IRCServer {
  address: String,
  port: u16,
}

impl IRCServer {
  pub fn new(address: String, port: u16) -> IRCServer {
    IRCServer {
      address: address,
      port: port,
    }
  }

  pub fn start(&self) {
    let listener = TcpListener::bind(self.address.as_slice(), self.port);

    let mut acceptor = listener.listen();

    for stream in acceptor.incoming() {
      match stream {
        Err(e) => { println!("Fail!"); break }
        Ok(stream) => spawn(proc() {
          IRCServer::handle_client(stream)
        })
      }
    }

    drop(acceptor);
  }

  fn handle_client(mut stream: TcpStream) {
    println!("Connected")
  }
}

