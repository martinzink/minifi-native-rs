use crate::{Logger, ProcessContext};

struct TransformResult<FlowFileType> {
    flow_file: FlowFileType,
}

pub trait ConstFlowFileTransform {
    fn transform<Context: ProcessContext, LoggerImpl: Logger>(
        &self,
        context: &mut Context,
        flow_file: Context::FlowFile,
        logger: &LoggerImpl,
    ) -> TransformResult<Context::FlowFile>;
}

pub trait MutFlowFileTransform {
    fn transform<Context: ProcessContext, LoggerImpl: Logger>(
        &mut self,
        context: &mut Context,
        flow_file: Context::FlowFile,
        logger: &LoggerImpl,
    ) -> TransformResult<Context::FlowFile>;
}
