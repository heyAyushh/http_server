use crate::http::{ParseError, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::{io::Read, net::TcpListener};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            let res = listener.accept();

            match res {
                Ok((mut stream, _addr)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Recieved a request {}", String::from_utf8_lossy(&buffer));

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(&request).send(&mut stream).ok();
                                }
                                Err(e) => {
                                    handler.handle_bad_request(&e).send(&mut stream).ok();
                                }
                            }
                        }
                        Err(e) => println!("Failed to read from connection!  {} ", e),
                    }
                }
                Err(e) => println!("Failed to establish connection! {}", e),
            }
        }
    }
}
