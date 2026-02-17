use crate::{MinifiError, ProcessSession};

pub enum Content<'a> {
    Buffer(Vec<u8>),
    Stream(Box<dyn std::io::Read + 'a>),
}

impl From<Vec<u8>> for Content<'_> {
    fn from(v: Vec<u8>) -> Self {
        Content::Buffer(v)
    }
}

impl From<String> for Content<'_> {
    fn from(s: String) -> Self {
        Content::Buffer(s.into_bytes())
    }
}

impl Content<'_> {
    pub(crate) fn write_to_flow_file<PS>(
        self,
        mut flow_file: &mut PS::FlowFile,
        session: &mut PS,
    ) -> Result<(), MinifiError>
    where
        PS: ProcessSession,
    {
        match self {
            Content::Buffer(buffer) => session.write(&mut flow_file, &buffer),
            Content::Stream(mut stream) => {
                session.write_in_batches(&mut flow_file, |buffer| {
                    match stream.read(buffer) {
                        Ok(0) => None, // EOF
                        Ok(n) => Some(n),
                        Err(_e) => None, // Signal failure/EOF
                    }
                })
            }
        }
    }
}
