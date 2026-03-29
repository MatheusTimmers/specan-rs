use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::SpectrumAnalyzer,
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct AverageMaximumOutputPower;

impl Assay for AverageMaximumOutputPower {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        instrument.set_span(config.bandwidth_mhz * 1.5)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_rbw(1000.0)?;
        instrument.set_vbw(3000.0)?;
        instrument.set_trace_mode("AVER")?;
        instrument.set_detector("RMS")?;
        instrument.set_sweep_auto(true)?;
        instrument.set_continuous_sweep(false)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_secs(10));

        let power = instrument.get_channel_power(config.bandwidth_mhz)?;

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Average Maximum Output Power".to_string(),
            measurements: vec![power],
            screenshot,
        })
    }
}
