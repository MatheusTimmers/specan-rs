use crate::{error::SpecanError, instrument::{Measurement,SpectrumAnalyzer}};
pub mod wifi;
pub mod bluetooth;
pub mod occupied_bandwidth;
pub mod maximum_peak_power;

use occupied_bandwidth::OccupiedBandwidth;
use maximum_peak_power::MaximumPeakPower;
use wifi::average_maximum_output_power::AverageMaximumOutputPower;
use wifi::power_spectral_density::PowerSpectralDensity;
use wifi::average_power_spectral_density::AveragePowerSpectralDensity;
use bluetooth::output_power::OutputPower;
use bluetooth::peak_power_spectral_density::PeakPowerSpectralDensity;

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

pub enum AssayKind {
    OccupiedBandwidth(OccupiedBandwidth),
    MaximumPeakPower(MaximumPeakPower),
    AverageMaximumOutputPower(AverageMaximumOutputPower),
    PowerSpectralDensity(PowerSpectralDensity),
    AveragePowerSpectralDensity(AveragePowerSpectralDensity),
    OutputPower(OutputPower),
    PeakPowerSpectralDensity(PeakPowerSpectralDensity),
}

impl AssayKind {
    pub fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        match self {
            AssayKind::OccupiedBandwidth(a) => a.run(instrument, config),
            AssayKind::MaximumPeakPower(a) => a.run(instrument, config),
            AssayKind::AverageMaximumOutputPower(a) => a.run(instrument, config),
            AssayKind::PowerSpectralDensity(a) => a.run(instrument, config),
            AssayKind::AveragePowerSpectralDensity(a) => a.run(instrument, config),
            AssayKind::OutputPower(a) => a.run(instrument, config),
            AssayKind::PeakPowerSpectralDensity(a) => a.run(instrument, config),
        }
    }
}
