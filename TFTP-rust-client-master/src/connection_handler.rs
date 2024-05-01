use std::io::prelude::*;
use std::net::TcpStream;
use crate::encdec::EenDec;
use crate::encdec::Opcode;

pub struct ConnectionHandler {
    host: String,
    port: u16,
    stream: TcpStream,
    enc_dec: EenDec,
}

impl ConnectionHandler {
    pub  fn new(host: String, port: u16) -> ConnectionHandler {
        let stream = TcpStream::connect(format!("{}:{}", host, port)).
            expect("Could not connect to the server make sure it is running! and the ip and port are correct.");
        stream.set_nonblocking(true).unwrap();
        println!("Connected to the server!");
        ConnectionHandler {
            host,
            port,
            stream,
            enc_dec: EenDec::new(),
        }
    }

    pub fn send(&mut self, bytes: Vec<u8>) {
        self.stream.write(self.enc_dec.encode(bytes).as_slice()).unwrap();
    }

    pub fn receive(&mut self) -> Option<(Vec<u8>, Opcode)> {
    //     read one byte at a time
        let mut buffer = [0; 1];
        if let Ok(_) = self.stream.read_exact(&mut buffer){
            self.enc_dec.decode(buffer[0])
        } else {
            None
        }

    }
}