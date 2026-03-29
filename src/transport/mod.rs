use crate::error::SpecanError;
mod tcp_transport;
pub use tcp_transport::TcpTransport;

pub trait Transport {
    fn send(&mut self, msg: &str) -> Result<(), SpecanError>;
    fn query(&mut self, msg: &str) -> Result<String, SpecanError>;
}
