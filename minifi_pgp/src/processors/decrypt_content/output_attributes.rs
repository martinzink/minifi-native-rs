use minifi_native::OutputAttribute;

pub(crate) const LITERAL_DATA_FILENAME: OutputAttribute = OutputAttribute {
    name: "pgp.literal.data.filename",
    relationships: &["success"],
    description: "Filename from decrypted Literal Data",
};

pub(crate) const LITERAL_DATA_MODIFIED: OutputAttribute = OutputAttribute {
    name: "pgp.literal.data.modified",
    relationships: &["success"],
    description: "Modified Date from decrypted Literal Data",
};
