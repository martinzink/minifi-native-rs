mod properties;
mod relationships;

use minifi_native::{Concurrent, ConcurrentOnTrigger, LogLevel, Logger, MinifiError, OnTriggerResult, ProcessContext, ProcessSession, Processor};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "PascalCase")]
enum KamikazeBehaviour {
    ReturnErr,
    ReturnOk,
    Panic,
}

#[derive(Debug)]
struct KamikazeProcessor<L: Logger> {
    logger: L,
    on_trigger_behaviour: KamikazeBehaviour,
    read_behaviour: Option<KamikazeBehaviour>,
    write_behaviour: Option<KamikazeBehaviour>,
}

impl<L: Logger> Processor<L> for KamikazeProcessor<L> {
    type Threading = Concurrent;
    fn new(logger: L) -> Self {
        Self {
            logger,
            on_trigger_behaviour: KamikazeBehaviour::ReturnOk,
            read_behaviour: None,
            write_behaviour: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        self.on_trigger_behaviour = context
            .get_property(&properties::ON_TRIGGER_BEHAVIOUR, None)?
            .expect("required property")
            .parse::<KamikazeBehaviour>()?;
        self.read_behaviour = context
            .get_property(&properties::READ_BEHAVIOUR, None)?
            .map(|s| s.parse::<KamikazeBehaviour>().unwrap());
        self.write_behaviour = context
            .get_property(&properties::WRITE_BEHAVIOUR, None)?
            .map(|s| s.parse::<KamikazeBehaviour>().unwrap());

        let on_schedule_behaviour = context
            .get_property(&properties::ON_SCHEDULE_BEHAVIOUR, None)?
            .expect("required property")
            .parse::<KamikazeBehaviour>()?;

        match on_schedule_behaviour {
            KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
            KamikazeBehaviour::ReturnOk => Ok(()),
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor panic")
            }
        }
    }
}

impl<L: Logger> ConcurrentOnTrigger<L> for KamikazeProcessor<L> {
    fn on_trigger<PC, PS>(&self, _context: &mut PC, session: &mut PS) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        if let Some(read_behaviour) = self.read_behaviour
            && let Some(flow_file) = session.get()
        {
            let _read = session.read_in_batches(&flow_file, 1, |_data| match read_behaviour {
                KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
                KamikazeBehaviour::ReturnOk => Ok(()),
                KamikazeBehaviour::Panic => {
                    panic!("KamikazeProcessor panic")
                }
            });
            session.transfer(flow_file, relationships::SUCCESS.name);
        }

        match self.on_trigger_behaviour {
            KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
            KamikazeBehaviour::ReturnOk => Ok(OnTriggerResult::Ok),
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor panic")
            }
        }
    }
}

#[cfg(not(test))]
mod register_ctor;

#[cfg(test)]
mod tests;
