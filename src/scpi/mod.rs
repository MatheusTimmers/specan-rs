use crate::transport::TcpTransport;

pub struct Scpi {
    socket: TcpTransport,
}

impl Scpi {
    pub fn new(socket: TcpTransport) -> Scpi {
        Scpi { socket }
    }
    
    pub fn query(&mut self, cmd: &str) -> String {
      self.socket.query(cmd)
    }

    pub fn write(&mut self, cmd: &str) {
        self.socket.send(cmd)
    }

    pub fn idn(&mut self) -> String {
        self.socket.query("*IDN?")
    }

    pub fn reset(&mut self) {
        self.socket.send("*RST");
    }

    pub fn wait(&mut self) {
        self.socket.send("*WAI");
    }
}
