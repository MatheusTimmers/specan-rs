use crate::scpi::Scpi;
use crate::transport::Transport;
use crate::instrument::{Measurement, SpectrumAnalyzer};
use crate::error::SpecanError;

pub struct N9010a<T: Transport> {
    client: Scpi<T>,
}

impl<T: Transport> N9010a<T> {
    pub fn new(transport: T) -> Self {
        N9010a { client: Scpi::new(transport) }
    }
}

impl<T: Transport> SpectrumAnalyzer for N9010a<T> {
    // frequency
    fn set_center_frequency(&mut self, mhz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":FREQ:CENT {mhz} MHz"))?;
        Ok(())
    }

    fn set_span(&mut self, mhz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":FREQ:SPAN {mhz} MHz"))?;
        Ok(())
    }

    fn set_start_frequency(&mut self, mhz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":FREQ:STAR {mhz} MHz"))?;
        Ok(())
    }

    fn set_stop_frequency(&mut self, mhz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":FREQ:STOP {mhz} MHz"))?;
        Ok(())
    }

    // amplitude
    fn set_reference_level(&mut self, dbm: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":DISP:WIND:TRAC:Y:SCAL:RLEV {dbm} dBm"))?;
        Ok(())
    }

    fn set_attenuation(&mut self, db: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":SENS:POW:RF:ATT {db} dB"))?;
        Ok(())
    }

    fn set_power_unit(&mut self, unit: &str) -> Result<(), SpecanError> {
        self.client.write(&format!(":UNIT:POW {unit}"))?;
        Ok(())
    }

    // bandwidth
    fn set_rbw(&mut self, khz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":BAND {khz} kHz"))?;
        Ok(())
    }

    fn set_vbw(&mut self, khz: f64) -> Result<(), SpecanError> {
        self.client.write(&format!(":BAND:VID {khz} kHz"))?;
        Ok(())
    }

    fn set_sweep_auto(&mut self, auto: bool) -> Result<(), SpecanError> {
        let val = if auto { "ON" } else { "OFF" };
        self.client.write(&format!(":SWE:TIME:AUTO {val}"))?;
        Ok(())
    }

    // trace
    fn set_trace_mode(&mut self, mode: &str) -> Result<(), SpecanError> {
        self.client.write(&format!(":TRAC:TYPE {mode}"))?;
        Ok(())
    }

    fn set_detector(&mut self, detector: &str) -> Result<(), SpecanError> {
        self.client.write(&format!(":SENS:DET:TRAC {detector}"))?;
        Ok(())
    }

    // measurements
    fn get_obw(&mut self, occupancy_percent: f64, xdb_down: f64) -> Result<Measurement, SpecanError> {
        self.client.write(":CONF:OBW")?;
        self.client.write(&format!(":SENS:OBW:PERC {occupancy_percent}"))?;
        self.client.write(&format!(":SENS:OBW:XDB {xdb_down} DB"))?;
        self.client.write(":INIT:IMM")?;
        let result = self.client.query(":FETC:OBW?")?.parse::<f64>().map_err(|e| SpecanError::Parser(e.to_string()))?;
        Ok(Measurement { value: result, unit: "Hz".to_string() })
    }

    fn get_channel_power(&mut self, integration_bw_mhz: f64) -> Result<Measurement, SpecanError> {
        self.client.write(":CONF:CHP")?;
        self.client.write(&format!(":CHP:BAND:INT {integration_bw_mhz} MHz"))?;
        self.client.write(":INIT:IMM")?;
        let result = self.client.query(":MEAS:CPOW?")?.parse::<f64>().map_err(|e| SpecanError::Parser(e.to_string()))?;
        Ok(Measurement { value: result, unit: "dBm".to_string() })
    }

    fn get_peak_power(&mut self) -> Result<Measurement, SpecanError> {
        self.client.write(":CALC:MARK:FUNC:TYPE MAX")?;
        self.client.write(":CALC:MARK:FUNC:EXEC")?;
        let result = self.client.query(":CALC:MARK:Y?")?.parse::<f64>().map_err(|e| SpecanError::Parser(e.to_string()))?;
        Ok(Measurement { value: result, unit: "dBm".to_string() })
    }
}
