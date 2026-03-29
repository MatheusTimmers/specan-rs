use crate::{error::SpecanError, instrument::{Measurement,SpectrumAnalyzer}};
pub mod wifi;
pub mod bluetooth;
pub mod occupied_bandwidth;
pub mod maximum_peak_power;

pub struct AssayResult {
    pub name: String,
    pub measurements: Vec<Measurement>,
    pub screenshot: Option<Vec<u8>>,
}

pub struct AssayConfig {
    pub center_frequency_mhz: f64,
    pub bandwidth_mhz: f64,
    pub attenuation_db: f64,
    pub reference_level_dbm: f64,
    pub capture_screen: bool,
}

pub trait Assay {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError>;
}
