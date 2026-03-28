use crate::scpi::Scpi;
use crate::transport::TcpTransport;
use crate::instrument::SpectrumAnalyzer;

pub struct N9010a {
    client: Scpi,
}

impl N9010a{
    pub fn connect(ip: &str, port: u16) -> N9010a {
        let socket = TcpTransport::connect(ip, port);
        let client = Scpi::new(socket);
        N9010a { client }
    }
}

impl SpectrumAnalyzer for N9010a {
    fn set_center_frequency(&mut self, hz: f64) {
        self.client.write(&format!(":SENS:FREQ:CENT {hz}"));
    }

    fn set_span(&mut self, span: f64) {
        self.client.write(&format!(":SENS:FREQ:SPAN {span}"));
    }

    fn set_reference_level(&mut self, ref_lev: f64) {
        self.client.write(&format!(":DISP:WIND:TRAC:Y:SCAL:RLEV {ref_lev}"));
    }

    fn set_attenuation(&mut self, att: f64) {
        self.client.write(&format!(":SENS:POW:RF:ATT {att}"));
    }
}
