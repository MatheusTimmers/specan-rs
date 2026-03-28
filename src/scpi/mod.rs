use crate::transport::Transport;
use crate::error::SpecanError;

pub struct Scpi<T: Transport> {
    transport: T,
}

impl<T: Transport> Scpi<T> {
    pub fn new(transport: T) -> Scpi<T>{
        Scpi { transport }
    }
    
    pub fn query(&mut self, cmd: &str) -> Result<String, SpecanError> {
      self.transport.query(cmd)
    }

    pub fn write(&mut self, cmd: &str) -> Result<(), SpecanError>{
        self.transport.send(cmd)
    }

    pub fn idn(&mut self) -> Result<String, SpecanError> {
        self.transport.query("*IDN?")
    }

    pub fn reset(&mut self) -> Result<(), SpecanError> {
        self.transport.send("*RST")
    }

    pub fn wait(&mut self) -> Result<(), SpecanError> {
        self.transport.send("*WAI")
    }
}
