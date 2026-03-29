use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::SpectrumAnalyzer,
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct OutputPower;

impl Assay for OutputPower {
    fn run<A: SpectrumAnalyzer>(&mut self, instrument: &mut A, config: &AssayConfig) -> Result<AssayResult, SpecanError> {
        instrument.reset()?;
        instrument.set_center_frequency(config.center_frequency_mhz)?;
        instrument.set_span(config.bandwidth_mhz * 1.5)?;
        instrument.set_attenuation(config.attenuation_db)?;
        instrument.set_reference_level(config.reference_level_dbm)?;
        instrument.set_rbw(100.0)?;
        instrument.set_vbw(300.0)?;
        instrument.set_trace_mode("MAXH")?;
        instrument.set_detector("POS")?;
        instrument.set_sweep_auto(true)?;
        instrument.set_continuous_sweep(false)?;
        instrument.initiate_sweep()?;

        thread::sleep(Duration::from_secs(15));

        // Step 1: get OBW at -26 dB to find the real occupied bandwidth
        let obw = instrument.get_obw(99.0, 26.0)?;
        let integration_bw_mhz = obw.value / 1_000_000.0;

        // Step 2: measure channel power using the actual occupied bandwidth
        let power = instrument.get_channel_power(integration_bw_mhz)?;

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Output Power".to_string(),
            measurements: vec![power],
            screenshot,
        })
    }
}
