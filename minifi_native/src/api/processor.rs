use crate::api::{RawProcessor, ThreadingModel};
use crate::{GetProperty, LogLevel, Logger, MinifiError, ProcessContext};
use std::marker::PhantomData;

pub trait Schedule {
    fn schedule<P: GetProperty, L: Logger>(context: &P, logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized;

    fn unschedule(&mut self) {}
}

pub trait CalculateMetrics {
    fn calculate_metrics(&self) -> Vec<(String, f64)> {
        vec![]
    }
}

pub trait AdvancedProcessorFeatures {
    fn restore(&self) -> bool;
    fn get_trigger_when_empty(&self) -> bool;
    fn is_work_available(&self) -> bool;
}

pub struct Processor<Impl, Kind, T, L>
where
    Impl: Schedule + CalculateMetrics + AdvancedProcessorFeatures,
    T: ThreadingModel,
    L: Logger,
{
    pub(crate) logger: L,
    pub(crate) scheduled_impl: Option<Impl>,
    threading_model: PhantomData<T>,
    flow_file_type: PhantomData<Kind>,
}

impl<Impl, Kind, T, L> RawProcessor for Processor<Impl, Kind, T, L>
where
    Impl: Schedule + CalculateMetrics + AdvancedProcessorFeatures,
    T: ThreadingModel,
    L: Logger,
{
    type Threading = T;
    type LoggerType = L;

    fn new(logger: Self::LoggerType) -> Self {
        Self {
            logger,
            scheduled_impl: None,
            threading_model: PhantomData,
            flow_file_type: PhantomData,
        }
    }

    fn restore(&self) -> bool {
        self.scheduled_impl
            .as_ref()
            .and_then(|i| Some(i.restore()))
            .unwrap_or(false)
    }

    fn get_trigger_when_empty(&self) -> bool {
        self.scheduled_impl
            .as_ref()
            .and_then(|i| Some(i.get_trigger_when_empty()))
            .unwrap_or(false)
    }

    fn is_work_available(&self) -> bool {
        self.scheduled_impl
            .as_ref()
            .and_then(|i| Some(i.is_work_available()))
            .unwrap_or(false)
    }

    fn log(&self, log_level: LogLevel, args: std::fmt::Arguments) {
        self.logger.log(log_level, args);
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
            // this seems to normal so no need for warnings
            vec![]
        }
    }
}
