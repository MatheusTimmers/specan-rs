use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct OccupancyTime {
    pub sweep_time_ms: u64,
}

impl Assay for OccupancyTime {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        // Zero span: time domain measurement
        instrument.set_span(0.0)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_rbw(1000.0)?;
        instrument.set_vbw(3000.0)?;
        instrument.set_trace_mode("WRIT")?;
        instrument.set_detector("POS")?;
        instrument.set_sweep_auto(false)?;
        instrument.set_continuous_sweep(false)?;

        // Configure sweep window to cover the expected burst period
        let sweep_time_s = self.sweep_time_ms as f64 / 1000.0;
        instrument.set_rbw(1000.0)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_millis(self.sweep_time_ms + 500));

        // In zero span, marker X returns time in seconds
        let burst_time_s = instrument.get_marker_time(1)?;
        let burst_time_ms = burst_time_s * 1000.0;

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Occupancy Time".to_string(),
            measurements: vec![
                Measurement { value: burst_time_ms, unit: "ms".to_string() },
                Measurement { value: sweep_time_s * 1000.0, unit: "ms (window)".to_string() },
            ],
            screenshot,
        })
    }
}
