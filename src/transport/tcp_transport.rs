use std::time::Duration;
use std::{io::{BufRead, BufReader, Write}, net::TcpStream};
use crate::error::SpecanError;
use crate::transport::Transport;

pub struct TcpTransport {
    stream: TcpStream,
    reader: BufReader<TcpStream>
}

impl Transport for TcpTransport {
    fn query(&mut self, cmd: &str) -> Result<String, SpecanError> {
        self.send(cmd)?;
        self.recv()
    }

    fn send(&mut self, cmd: &str) -> Result<(), SpecanError> {
        let msg = format!("{cmd}\n");
        self.stream.write_all(msg.as_bytes()).map_err(|e| SpecanError::Connection(e.to_string()))?;

        Ok(())
    }
}

impl TcpTransport {
    pub fn connect(ip: &str, port: u16, timeout_ms: u64) -> Result<TcpTransport, SpecanError> {
        let addr = format!("{ip}:{port}");
        let stream = TcpStream::connect(addr).map_err(|e| SpecanError::Connection(e.to_string()))?;

        let timeout = Some(Duration::from_millis(timeout_ms));
        stream.set_read_timeout(timeout).map_err(|e| SpecanError::Connection(e.to_string()))?;
        stream.set_write_timeout(timeout).map_err(|e| SpecanError::Connection(e.to_string()))?;

        let reader = BufReader::new(stream.try_clone().map_err(|e| SpecanError::Connection(e.to_string()))?);

        Ok(TcpTransport { stream, reader})
    }

    fn recv(&mut self) -> Result<String, SpecanError> {
        let mut line = String::new();
        self.reader.read_line(&mut line).map_err(|e| SpecanError::Connection(e.to_string()))?;

        Ok(line)
    }
}
