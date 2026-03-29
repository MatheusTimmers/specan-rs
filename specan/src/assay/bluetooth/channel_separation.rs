use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct ChannelSeparation {
    pub channel_count: u32,
}

impl Assay for ChannelSeparation {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        // Wide span to capture all hop channels (Bluetooth uses 79 MHz total)
        instrument.set_span(100.0)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_sweep_auto(true)?;
        instrument.set_trace_mode("MAXH")?;
        instrument.set_detector("POS")?;
        instrument.set_continuous_sweep(false)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_secs(10));

        let markers = instrument.get_markers(self.channel_count)?;

        if markers.len() < 2 {
            return Err(SpecanError::Instrument("need at least 2 markers to compute separation".to_string()));
        }

        let frequencies: Vec<f64> = markers.iter().map(|m| m.frequency_hz).collect();
        let min_freq = frequencies.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_freq = frequencies.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let separation_mhz = (max_freq - min_freq) / 1_000_000.0;

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Channel Separation".to_string(),
            measurements: vec![Measurement { value: separation_mhz, unit: "MHz".to_string() }],
            screenshot,
        })
    }
}
