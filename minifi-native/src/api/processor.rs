use crate::api::RawThreadingModel;
use crate::{LogLevel, Logger, MinifiError, ProcessContext, RawProcessor};
use std::marker::PhantomData;
use crate::c_ffi::CffiLogger;

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

pub struct Processor<Impl, Kind, T>
where
    Impl: Schedule + CalculateMetrics,
    T: RawThreadingModel,
{
    pub(crate) logger: CffiLogger,
    pub(crate) scheduled_impl: Option<Impl>,
    threading_model: PhantomData<T>,
    flow_file_type: PhantomData<Kind>,
}

impl<Impl, Kind, T> RawProcessor for Processor<Impl, Kind, T>
where
    Impl: Schedule + CalculateMetrics,
    T: RawThreadingModel,
{
    type Threading = T;

    fn new(logger: CffiLogger) -> Self {
        Self {
            logger,
            scheduled_impl: None,
            threading_model: PhantomData,
            flow_file_type: PhantomData,
        }
    }

    fn restore(&self) -> bool {
        todo!()
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.scheduled_impl = Some(Impl::schedule(context, &self.logger)?);
        Ok(())
    }

    fn on_unschedule(&mut self) {
        if let Some(ref mut scheduled_impl) = self.scheduled_impl {
            scheduled_impl.unschedule()
        }
    }

    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        if let Some(ref scheduled_impl) = self.scheduled_impl {
            scheduled_impl.calculate_metrics()
        } else {
            self.logger
                .warn("Calculating metrics before processor is scheduled.");
            vec![]
        }
    }
}
