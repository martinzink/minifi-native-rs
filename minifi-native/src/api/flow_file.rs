pub trait FlowFile {
    fn set_attribute(&mut self, attribute_name: &str, attribute_value: &str);
}
