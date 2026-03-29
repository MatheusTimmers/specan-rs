use std::time::Duration;
use std::{io::{BufRead, BufReader, Write}, net::TcpStream};
use crate::error::SpecanError;
use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
}

impl Transport for TcpTransport {
    fn query(&mut self, cmd: &str) -> Result<String, SpecanError> {
        self.send(cmd)?;
        self.recv()
    }

    fn send(&mut self, cmd: &str) -> Result<(), SpecanError> {
        let msg = format!("{cmd}\n");
        self.stream.write_all(msg.as_bytes())?;
        Ok(())
    }
}

impl TcpTransport {
    pub fn connect(ip: &str, port: u16, timeout_ms: u64) -> Result<TcpTransport, SpecanError> {
        let addr = format!("{ip}:{port}");
        let stream = TcpStream::connect(addr)?;

        let timeout = Some(Duration::from_millis(timeout_ms));
        stream.set_read_timeout(timeout)?;
        stream.set_write_timeout(timeout)?;

        let reader = BufReader::new(stream.try_clone()?);

        Ok(TcpTransport { stream, reader })
    }

    fn recv(&mut self) -> Result<String, SpecanError> {
        let mut line = String::new();
        self.reader.read_line(&mut line)?;
        Ok(line)
    }
}
