use crate::error::SpecanError;
mod n9010a;
pub use n9010a::N9010a;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Measurement {
    pub value: f64,
    pub unit: String,
}

pub trait SpectrumAnalyzer {
    // frequency
    fn set_center_frequency(&mut self, mhz: f64) -> Result<(), SpecanError>;
    fn set_span(&mut self, mhz: f64) -> Result<(), SpecanError>;
    fn set_start_frequency(&mut self, mhz: f64) -> Result<(), SpecanError>;
    fn set_stop_frequency(&mut self, mhz: f64) -> Result<(), SpecanError>;

    // amplitude
    fn set_reference_level(&mut self, dbm: f64) -> Result<(), SpecanError>;
    fn set_attenuation(&mut self, db: f64) -> Result<(), SpecanError>;
    fn set_power_unit(&mut self, unit: &str) -> Result<(), SpecanError>;

    // bandwidth
    fn set_rbw(&mut self, khz: f64) -> Result<(), SpecanError>;
    fn set_vbw(&mut self, khz: f64) -> Result<(), SpecanError>;
    fn set_sweep_auto(&mut self, auto: bool) -> Result<(), SpecanError>;

    // trace
    fn set_trace_mode(&mut self, mode: &str) -> Result<(), SpecanError>;
    fn set_detector(&mut self, detector: &str) -> Result<(), SpecanError>;

    // measurements
    fn get_obw(&mut self, occupancy_percent: f64, xdb_down: f64) -> Result<Measurement, SpecanError>;
    fn get_channel_power(&mut self, integration_bw_mhz: f64) -> Result<Measurement, SpecanError>;
    fn get_peak_power(&mut self) -> Result<Measurement, SpecanError>;
    fn get_peak_markers(&mut self, count: u32) -> Result<Vec<Measurement>, SpecanError>;
    fn get_sweep_time(&mut self) -> Result<f64, SpecanError>;

    // sweep control
    fn initiate_sweep(&mut self) -> Result<(), SpecanError>;
    fn set_continuous_sweep(&mut self, on: bool) -> Result<(), SpecanError>;

    // screen
    fn capture_screen(&mut self) -> Result<Vec<u8>, SpecanError>;

    // reset
    fn reset(&mut self) -> Result<(), SpecanError>;
}
