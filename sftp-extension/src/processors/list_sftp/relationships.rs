use minifi_native::Relationship;

pub(crate) const SUCCESS: Relationship = Relationship {
    name: "success",
    description: "All FlowFiles that are received are routed to success",
};
