use specan::{
    error::SpecanError,
    transport::Transport,
    instrument::N9010a,
    session::Session,
    runner::Runner,
    assay::{AssayConfig, AssayKind},
    assay::occupied_bandwidth::OccupiedBandwidth,
    assay::maximum_peak_power::MaximumPeakPower,
    assay::wifi::average_maximum_output_power::AverageMaximumOutputPower,
    assay::wifi::power_spectral_density::PowerSpectralDensity,
    assay::wifi::average_power_spectral_density::AveragePowerSpectralDensity,
    assay::bluetooth::output_power::OutputPower,
    assay::bluetooth::peak_power_spectral_density::PeakPowerSpectralDensity,
};

struct MockTransport {
    responses: Vec<String>,
}

impl Transport for MockTransport {
    fn send(&mut self, _cmd: &str) -> Result<(), SpecanError> {
        Ok(())
    }

    fn query(&mut self, _cmd: &str) -> Result<String, SpecanError> {
        Ok(self.responses.remove(0))
    }
}

fn make_runner(responses: Vec<String>) -> Runner<N9010a<MockTransport>> {
    let transport = MockTransport { responses };
    let instrument = N9010a::new(transport);
    let session = Session::new(instrument);
    Runner::new(session)
}

fn base_config() -> AssayConfig {
    AssayConfig {
        center_frequency_mhz: 2437.0,
        bandwidth_mhz: 20.0,
        attenuation_db: 10.0,
        reference_level_dbm: 0.0,
        capture_screen: false,
    }
}

#[test]
fn test_occupied_bandwidth() {
    let mut runner = make_runner(vec!["20000.0".to_string()]);
    let mut assays = vec![AssayKind::OccupiedBandwidth(OccupiedBandwidth { xdb_down: 26 })];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, 20.0);
    assert_eq!(result.measurements[0].unit, "kHz");
}

#[test]
fn test_maximum_peak_power() {
    let mut runner = make_runner(vec!["-30.5".to_string()]);
    let mut assays = vec![AssayKind::MaximumPeakPower(MaximumPeakPower)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, -30.5);
    assert_eq!(result.measurements[0].unit, "dBm");
}

#[test]
fn test_average_maximum_output_power() {
    let mut runner = make_runner(vec!["-25.0".to_string()]);
    let mut assays = vec![AssayKind::AverageMaximumOutputPower(AverageMaximumOutputPower)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, -25.0);
    assert_eq!(result.measurements[0].unit, "dBm");
}

#[test]
fn test_power_spectral_density() {
    let mut runner = make_runner(vec![]);
    let mut assays = vec![AssayKind::PowerSpectralDensity(PowerSpectralDensity)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert!(result.measurements.is_empty());
    assert!(result.screenshot.is_none());
}

#[test]
fn test_average_power_spectral_density() {
    let mut runner = make_runner(vec!["-50.0".to_string()]);
    let mut assays = vec![AssayKind::AveragePowerSpectralDensity(AveragePowerSpectralDensity)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, -50.0);
    assert_eq!(result.measurements[0].unit, "dBm/Hz");
}

#[test]
fn test_output_power() {
    // first query: OBW in Hz, second query: channel power in dBm
    let mut runner = make_runner(vec!["1000000.0".to_string(), "-28.0".to_string()]);
    let mut assays = vec![AssayKind::OutputPower(OutputPower)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, -28.0);
    assert_eq!(result.measurements[0].unit, "dBm");
}

#[test]
fn test_peak_power_spectral_density() {
    let mut runner = make_runner(vec!["-45.0".to_string()]);
    let mut assays = vec![AssayKind::PeakPowerSpectralDensity(PeakPowerSpectralDensity)];

    let results = runner.run_all(&mut assays, &base_config());

    let result = results[0].as_ref().unwrap();
    assert_eq!(result.measurements[0].value, -45.0);
    assert_eq!(result.measurements[0].unit, "dBm/Hz");
}
