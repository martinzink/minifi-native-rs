mod properties;
mod relationships;

use minifi_native::macros::ComponentIdentifier;
use minifi_native::{
    CalculateMetrics, ConstTrigger, Logger, MinifiError, OnTriggerResult, ProcessContext,
    ProcessSession, Schedule,
};
use strum_macros::{Display, EnumString, IntoStaticStr, VariantNames};

#[derive(Debug, Clone, Copy, PartialEq, Display, EnumString, VariantNames, IntoStaticStr)]
#[strum(serialize_all = "PascalCase", const_into_str)]
enum KamikazeBehaviour {
    ReturnErr,
    ReturnOk,
    Panic,
}

#[derive(Debug, ComponentIdentifier)]
pub(crate) struct KamikazeProcessorRs {
    on_trigger_behaviour: KamikazeBehaviour,
    read_behaviour: Option<KamikazeBehaviour>,
}

impl Schedule for KamikazeProcessorRs {
    fn schedule<P: ProcessContext, L: Logger>(context: &P, _logger: &L) -> Result<Self, MinifiError>
    where
        Self: Sized,
    {
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
            KamikazeBehaviour::ReturnOk => Ok(KamikazeProcessorRs {
                on_trigger_behaviour,
                read_behaviour,
            }),
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor::on_schedule panic")
            }
        }
    }
}

impl ConstTrigger for KamikazeProcessorRs {
    fn trigger<PC, PS, L>(
        &self,
        _context: &mut PC,
        session: &mut PS,
        _logger: &L,
    ) -> Result<OnTriggerResult, MinifiError>
    where
        PC: ProcessContext,
        PS: ProcessSession<FlowFile = PC::FlowFile>,
        L: Logger,
    {
        if let Some(read_behaviour) = self.read_behaviour
            && let Some(flow_file) = session.get()
        {
            let _read = session.read_in_batches(&flow_file, 1, |_data| match read_behaviour {
                KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
                KamikazeBehaviour::ReturnOk => Ok(()),
                KamikazeBehaviour::Panic => {
                    panic!("KamikazeProcessor::on_trigger panic")
                }
            });

            session.transfer(flow_file, relationships::SUCCESS.name);
        }

        match self.on_trigger_behaviour {
            KamikazeBehaviour::ReturnErr => Err(MinifiError::UnknownError),
            KamikazeBehaviour::ReturnOk => Ok(OnTriggerResult::Ok),
            KamikazeBehaviour::Panic => {
                panic!("KamikazeProcessor::on_trigger panic")
            }
        }
    }
}

impl CalculateMetrics for KamikazeProcessorRs {}

#[cfg(not(test))]
pub(crate) mod processor_definition;

#[cfg(test)]
mod tests;
