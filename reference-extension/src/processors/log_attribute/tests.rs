use super::*;
use minifi_native::{MockFlowFile, MockLogger, MockProcessContext, MockProcessSession};

#[test]
fn simple_test() {
    let mut processor = LogAttribute::new(MockLogger::new());
    let mut context = MockProcessContext::new();
    context
        .properties
        .insert(String::from("Log Level"), String::from("Warn"));
    context
        .properties
        .insert(String::from("Log Payload"), String::from("true"));
    context
        .properties
        .insert(String::from("FlowFiles To Log"), String::from("1"));

    processor
        .on_schedule(&context)
        .expect("The on_schedule should succeed");

    {
        let mut session = MockProcessSession::new();
        let mut input_ff = MockFlowFile::new();
        input_ff.content = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer facilisis diam sit amet nisl interdum, vitae interdum arcu viverra. Nam placerat mi in erat pellentesque, at ultrices orci faucibus. Cras sollicitudin iaculis posuere. Sed tempus, dolor nec lacinia suscipit, tellus odio venenatis odio, nec sollicitudin dolor augue non urna. Aliquam tincidunt viverra ipsum eget hendrerit. Suspendisse varius, augue vel fermentum varius, velit elit euismod lacus, a placerat purus est a lacus. Aenean nibh neque, consectetur hendrerit egestas vitae, commodo non quam. Nullam luctus tempor ante, sed tempus quam imperdiet in. Maecenas gravida erat orci, in consequat urna pretium nec. In sodales iaculis magna at vehicula. ".to_string();
        input_ff.attributes.insert(String::from("Source"), String::from("test"));
        input_ff.attributes.insert(String::from("Date"), String::from("today"));
        session.input_flow_files.push(input_ff);
        processor
            .on_trigger(&context, &mut session)
            .expect("The on_trigger should succeed");
        let expected =
"Logging for flow file
--------------------------------------------------
FlowFile Attributes Map Content
key:Date value:today
key:Source value:test
Payload:
Lorem ipsum dolor sit amet, consectetur adipiscing elit. Integer facilisis diam sit amet nisl interdum, vitae interdum arcu viverra. Nam placerat mi in erat pellentesque, at ultrices orci faucibus. Cras sollicitudin iaculis posuere. Sed tempus, dolor nec lacinia suscipit, tellus odio venenatis odio, nec sollicitudin dolor augue non urna. Aliquam tincidunt viverra ipsum eget hendrerit. Suspendisse varius, augue vel fermentum varius, velit elit euismod lacus, a placerat purus est a lacus. Aenean nibh neque, consectetur hendrerit egestas vitae, commodo non quam. Nullam luctus tempor ante, sed tempus quam imperdiet in. Maecenas gravida erat orci, in consequat urna pretium nec. In sodales iaculis magna at vehicula. --------------------------------------------------".to_string();
        assert!(
            processor
                .logger
                .logs
                .contains(&(LogLevel::Warn, expected))
        );
    }
}
