use crate::{Logger, MinifiError, ProcessContext};

pub trait Schedule {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized;

    fn unschedule(&mut self) {}
}

pub trait CalculateMetrics {
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}
