use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct PeakPowerSpectralDensity;

impl Assay for PeakPowerSpectralDensity {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        instrument.set_span(config.bandwidth_mhz * 1.5)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_rbw(3.0)?;
        instrument.set_vbw(10.0)?;
        instrument.set_trace_mode("MAXH")?;
        instrument.set_detector("POS")?;
        instrument.set_sweep_auto(true)?;
        instrument.set_continuous_sweep(false)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_secs(10));

        let peak = instrument.get_peak_markers(1)?.into_iter().next()
            .ok_or_else(|| SpecanError::Instrument("no marker returned".to_string()))?;
        let psd = Measurement { value: peak.value, unit: "dBm/Hz".to_string() };

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Peak Power Spectral Density".to_string(),
            measurements: vec![psd],
            screenshot,
        })
    }
}
