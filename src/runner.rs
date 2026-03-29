use std::time::Instant;
use tracing::{error, info, info_span};
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
        assays.iter_mut().map(|assay| {
            let name = assay.name();
            let span = info_span!("assay", name);
            let _enter = span.enter();

            info!("starting");
            let start = Instant::now();

            let result = assay.run(self.session.instrument(), config);

            match &result {
                Ok(_) => info!(elapsed_ms = start.elapsed().as_millis(), "completed"),
                Err(e) => error!(error = %e, "failed"),
            }

            result
        }).collect()
    }
}
