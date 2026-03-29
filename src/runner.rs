use crate::session::Session;
use crate::instrument::SpectrumAnalyzer;

pub struct Runner<T: SpectrumAnalyzer> {
    session: Session<T>,
}

impl<T: SpectrumAnalyzer> Runner<T> {
    pub fn new(session: Session<T>) -> Self {
        Self { session }
    }
}
