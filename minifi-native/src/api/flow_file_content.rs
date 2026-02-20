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
