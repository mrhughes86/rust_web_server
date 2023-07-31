use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
        addr: String,
    }

    impl Server {
        pub fn new(addr: String) -> Self {
            Self { addr }
        }
        pub fn run(self) {
            println!("Listening on {:?}", self.addr);

            let listener = TcpListener::bind(&self.addr).unwrap();

            loop {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        let mut buffer = [0; 1024]; // BUFFER SIZE MAY BE TOO SMALL FOR PRODUCTION READINESS.
                        match stream.read(&mut buffer) {
                            Ok(_) => {
                                println!("Connection Request: {}", String::from_utf8_lossy(&buffer)); // Converts bytes to strings even if invalid chars.
                                match Request::try_from(&buffer[..]) {
                                    Ok(request) => {},
                                    Err(e) => println!("Failed to parse request: {}", e)
                                }
                            },
                            Err(e) => println!("There was a connection issue:{}", e)
                        }
                    }
                    Err(e) => println!("Failed to establish a connection. {}", e)
                }
            }
        }
    }
