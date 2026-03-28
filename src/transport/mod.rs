use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

pub struct TcpTransport {
    stream: TcpStream,
    reader: BufReader<TcpStream>
}

impl TcpTransport {
    pub fn connect(ip: &str, port: u16) -> TcpTransport {
        let addr = format!("{ip}:{port}");
        let stream = TcpStream::connect(addr).unwrap();
        let reader = BufReader::new(stream.try_clone().unwrap());

        TcpTransport { stream, reader}
    }

    pub fn query(&mut self, cmd: &str) -> String{
        self.send(cmd);
        self.recv()
    }

    pub fn send(&mut self, cmd: &str){
        let msg = format!("{cmd}\n");
        self.stream.write_all(msg.as_bytes()).unwrap();
    }

    pub fn recv(&mut self) -> String {
        let mut line = String::new();
        self.reader.read_line(&mut line).unwrap();
        line
    }
}
