use crate::{DefaultLogger, MinifiError, ProcessContext};

pub trait Schedulable {
    fn schedule<P: ProcessContext>(context: &P, logger: &DefaultLogger) -> Result<Self, MinifiError> where Self: Sized;
    
    fn unschedule(&mut self) {}
}