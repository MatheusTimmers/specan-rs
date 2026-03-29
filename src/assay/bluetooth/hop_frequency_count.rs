use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct HopFrequencyCount {
    pub threshold_dbm: f64,
    pub max_markers: u32,
}

impl Assay for HopFrequencyCount {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        instrument.set_span(100.0)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_sweep_auto(true)?;
        instrument.set_trace_mode("MAXH")?;
        instrument.set_detector("POS")?;
        instrument.set_continuous_sweep(false)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_secs(10));

        let markers = instrument.get_markers(self.max_markers)?;
        let count = markers.iter().filter(|m| m.power_dbm >= self.threshold_dbm).count();

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Hop Frequency Count".to_string(),
            measurements: vec![Measurement { value: count as f64, unit: "channels".to_string() }],
            screenshot,
        })
    }
}
