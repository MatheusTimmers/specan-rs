use crate::session::Session;
use crate::instrument::SpectrumAnalyzer;
use crate::assay::{AssayConfig, AssayKind, AssayResult};
use crate::error::SpecanError;

pub struct Runner<T: SpectrumAnalyzer> {
    session: Session<T>,
}

impl<T: SpectrumAnalyzer> Runner<T> {
    pub fn new(session: Session<T>) -> Self {
        Self { session }
    }

    pub fn run_all(&mut self, assays: &mut Vec<AssayKind>, config: &AssayConfig) -> Vec<Result<AssayResult, SpecanError>> {
        assays.iter_mut()
            .map(|assay| assay.run(self.session.instrument(), config))
            .collect()
    }
}
