mod properties;
mod relationships;

use minifi_native::{
    Concurrent, RawMultiThreadedTrigger, DefaultLogger, LogLevel, Logger, MinifiError, OnTriggerResult,
    ProcessContext, ProcessSession, RawProcessor,
};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "PascalCase")]
enum KamikazeBehaviour {
    ReturnErr,
    ReturnOk,
    Panic,
}

#[derive(Debug)]
struct ScheduledMembers {
    on_trigger_behaviour: KamikazeBehaviour,
    read_behaviour: Option<KamikazeBehaviour>,
}

#[derive(Debug)]
pub(crate) struct KamikazeProcessor {
    logger: DefaultLogger,
    scheduled_members: Option<ScheduledMembers>,
}

impl RawProcessor for KamikazeProcessor {
    type Threading = Concurrent;
    fn new(logger: DefaultLogger) -> Self {
        Self {
            logger,
            scheduled_members: None,
        }
    }

    fn log(&self, log_level: LogLevel, message: &str) {
        self.logger.log(log_level, message);
    }

    fn on_schedule<P: ProcessContext>(&mut self, context: &P) -> Result<(), MinifiError> {
        let on_trigger_behaviour = context
            .get_property(&properties::ON_TRIGGER_BEHAVIOUR, None)?
            .expect("required property")
            .parse::<KamikazeBehaviour>()?;
        let read_behaviour = context
            .get_property(&properties::READ_BEHAVIOUR, None)?
            .map(|s| s.parse::<KamikazeBehaviour>().unwrap());

        let on_schedule_behaviour = context
            .get_property(&properties::ON_SCHEDULE_BEHAVIOUR, None)?
            .expect("required property")
            .parse::<KamikazeBehaviour>()?;

        match on_schedule_behaviour {
            KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
            KamikazeBehaviour::ReturnOk => {
                self.scheduled_members = Some(ScheduledMembers {
                    on_trigger_behaviour,
                    read_behaviour,
                });
                Ok(())
            }
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor panic")
            }
        }
    }
}

impl RawMultiThreadedTrigger for KamikazeProcessor {
    fn on_trigger<PC, PS>(
        &self,
        _context: &mut PC,
        session: &mut PS,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
    {
        let kamikaze_proc = self
            .scheduled_members
            .as_ref()
            .expect("on_schedule should create GetFileImpl");

        if let Some(read_behaviour) = kamikaze_proc.read_behaviour
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

        match kamikaze_proc.on_trigger_behaviour {
            KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
            KamikazeBehaviour::ReturnOk => Ok(OnTriggerResult::Ok),
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor panic")
            }
        }
    }
}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;
