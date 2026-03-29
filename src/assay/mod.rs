use crate::{error::SpecanError, instrument::{Measurement,SpectrumAnalyzer}};
pub mod wifi;
pub mod bluetooth;
pub mod occupied_bandwidth;
pub mod maximum_peak_power;
pub mod spurious_emissions;

use occupied_bandwidth::OccupiedBandwidth;
use maximum_peak_power::MaximumPeakPower;
use spurious_emissions::SpuriousEmissions;
use wifi::average_maximum_output_power::AverageMaximumOutputPower;
use wifi::power_spectral_density::PowerSpectralDensity;
use wifi::average_power_spectral_density::AveragePowerSpectralDensity;
use bluetooth::output_power::OutputPower;
use bluetooth::peak_power_spectral_density::PeakPowerSpectralDensity;
use bluetooth::channel_separation::ChannelSeparation;
use bluetooth::hop_frequency_count::HopFrequencyCount;
use bluetooth::occupancy_time::OccupancyTime;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssayResult {
    pub name: String,
    pub measurements: Vec<Measurement>,
    pub screenshot: Option<Vec<u8>>,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    SpuriousEmissions(SpuriousEmissions),
    AverageMaximumOutputPower(AverageMaximumOutputPower),
    PowerSpectralDensity(PowerSpectralDensity),
    AveragePowerSpectralDensity(AveragePowerSpectralDensity),
    OutputPower(OutputPower),
    PeakPowerSpectralDensity(PeakPowerSpectralDensity),
    ChannelSeparation(ChannelSeparation),
    HopFrequencyCount(HopFrequencyCount),
    OccupancyTime(OccupancyTime),
}

impl AssayKind {
    pub fn name(&self) -> &str {
        match self {
            AssayKind::OccupiedBandwidth(_) => "Occupied Bandwidth",
            AssayKind::MaximumPeakPower(_) => "Maximum Peak Power",
            AssayKind::SpuriousEmissions(_) => "Spurious Emissions",
            AssayKind::AverageMaximumOutputPower(_) => "Average Maximum Output Power",
            AssayKind::PowerSpectralDensity(_) => "Power Spectral Density",
            AssayKind::AveragePowerSpectralDensity(_) => "Average Power Spectral Density",
            AssayKind::OutputPower(_) => "Output Power",
            AssayKind::PeakPowerSpectralDensity(_) => "Peak Power Spectral Density",
            AssayKind::ChannelSeparation(_) => "Channel Separation",
            AssayKind::HopFrequencyCount(_) => "Hop Frequency Count",
            AssayKind::OccupancyTime(_) => "Occupancy Time",
        }
    }

    pub fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        match self {
            AssayKind::OccupiedBandwidth(a) => a.run(instrument, config),
            AssayKind::MaximumPeakPower(a) => a.run(instrument, config),
            AssayKind::AverageMaximumOutputPower(a) => a.run(instrument, config),
            AssayKind::PowerSpectralDensity(a) => a.run(instrument, config),
            AssayKind::AveragePowerSpectralDensity(a) => a.run(instrument, config),
            AssayKind::OutputPower(a) => a.run(instrument, config),
            AssayKind::PeakPowerSpectralDensity(a) => a.run(instrument, config),
            AssayKind::SpuriousEmissions(a) => a.run(instrument, config),
            AssayKind::ChannelSeparation(a) => a.run(instrument, config),
            AssayKind::HopFrequencyCount(a) => a.run(instrument, config),
            AssayKind::OccupancyTime(a) => a.run(instrument, config),
        }
    }
}
