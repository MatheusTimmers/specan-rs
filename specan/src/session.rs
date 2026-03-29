use crate::instrument::SpectrumAnalyzer;

pub struct Session<T: SpectrumAnalyzer> {
    instrument: T,
}

impl<T: SpectrumAnalyzer> Session<T> {
    pub fn new(instrument: T) -> Self {
        Self { instrument }
    }

    pub(crate) fn instrument(&mut self) -> &mut T {
        &mut self.instrument
    }
}
