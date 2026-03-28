mod n9010a;

pub trait SpectrumAnalyzer {
    fn set_center_frequency(&mut self, hz: f64);
    fn set_span(&mut self, hz: f64);
    fn set_reference_level(&mut self, dbm: f64);
    fn set_attenuation(&mut self, db: f64);
}
