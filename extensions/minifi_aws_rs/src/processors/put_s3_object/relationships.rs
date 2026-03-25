use minifi_native::Relationship;

pub const SUCCESS: Relationship = Relationship {
    name: "success",
    description: "FlowFiles are routed to success relationship",
};

pub const FAILURE: Relationship = Relationship {
    name: "failure",
    description: "FlowFiles are routed to failure relationship",
};
