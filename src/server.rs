use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};
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

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    Response::new(
                                        StatusCode::Ok,
                                        Some("<h1>CONNECTED.</h1>".to_string()),
                                    );

                                }
                                Err(e) =>{
                                    println!("Failed to parse request: {}", e);
                                    Response::new(StatusCode::BadRequest, None);
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => println!("There was a connection issue:{}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection. {}", e),
            }
        }
    }
}
