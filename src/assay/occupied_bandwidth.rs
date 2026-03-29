use std::thread;
use std::time::Duration;
use crate::{
    error::SpecanError,
    instrument::{Measurement, SpectrumAnalyzer},
    assay::{Assay, AssayConfig, AssayResult},
};

pub struct OccupiedBandwidth {
    pub xdb_down: u16,
}

impl Assay for OccupiedBandwidth {
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

        let obw = instrument.get_obw(99.0, self.xdb_down as f64)?;
        let obw_khz = Measurement { value: obw.value / 1000.0, unit: "kHz".to_string() };

        let screenshot = if config.capture_screen {
            Some(instrument.capture_screen()?)
        } else {
            None
        };

        Ok(AssayResult {
            name: "Occupied Bandwidth".to_string(),
            measurements: vec![obw_khz],
            screenshot,
        })
    }
}
