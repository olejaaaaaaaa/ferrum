
use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs}
};

pub struct Client {
    socket: TcpStream
}

impl Client {

    pub fn new<IP: ToSocketAddrs>(addr: IP) -> Self {
        let socket = TcpStream::connect(addr).unwrap();
        socket.set_nonblocking(true).unwrap();
        Client { socket }
    }

    pub fn send_data(&mut self, buf: &mut [u8]) {
        let _ = self.socket.write(buf);
    }

    pub fn recv_data(&mut self) -> Vec<u8> {
        let mut buf = vec![0; 1024];
        match self.socket.read(&mut buf) {
            Ok(0) => Vec::new(),
            Ok(n) => buf[..n].to_vec(),
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                Vec::new()
            }
            Err(e) => {
                eprintln!("Read error: {}", e);
                Vec::new()
            }
        }
    }
}
