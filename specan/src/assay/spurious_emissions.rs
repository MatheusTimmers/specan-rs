use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct SpuriousEmissions {
    pub ranges: Vec<(f64, f64)>, // (start_mhz, stop_mhz)
}

impl Assay for SpuriousEmissions {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        let mut measurements = Vec::new();

        for (start_mhz, stop_mhz) in &self.ranges {
            instrument.reset()?;
            instrument.set_start_frequency(*start_mhz)?;
            instrument.set_stop_frequency(*stop_mhz)?;
            instrument.set_attenuation(config.attenuation_db)?;
            instrument.set_reference_level(config.reference_level_dbm)?;
            instrument.set_sweep_auto(true)?;
            instrument.set_trace_mode("MAXH")?;
            instrument.set_detector("POS")?;
            instrument.set_continuous_sweep(false)?;
            instrument.initiate_sweep()?;

            thread::sleep(Duration::from_secs(5));

            let markers = instrument.get_peak_markers(1)?;
            let peak = markers.into_iter().next()
                .ok_or_else(|| SpecanError::Instrument("no marker returned".to_string()))?;
            measurements.push(Measurement { value: peak.value, unit: "dBm".to_string() });
        }

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Spurious Emissions".to_string(),
            measurements,
            screenshot,
        })
    }
}
