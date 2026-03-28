use crate::transport::TcpTransport;
use crate::error::SpecanError;

pub struct Scpi {
    socket: TcpTransport,
}

impl Scpi {
    pub fn new(socket: TcpTransport) -> Scpi {
        Scpi { socket }
    }
    
    pub fn query(&mut self, cmd: &str) -> Result<String, SpecanError> {
      self.socket.query(cmd)
    }

    pub fn write(&mut self, cmd: &str) -> Result<(), SpecanError>{
        self.socket.send(cmd)
    }

    pub fn idn(&mut self) -> Result<String, SpecanError> {
        self.socket.query("*IDN?")
    }

    pub fn reset(&mut self) -> Result<(), SpecanError> {
        self.socket.send("*RST")
    }

    pub fn wait(&mut self) -> Result<(), SpecanError> {
        self.socket.send("*WAI")
    }
}
