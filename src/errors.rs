#[derive(Debug)]
pub enum NodeError {
    ValueError(String),
    ParseError(String),
    IoError(std::io::Error),
    RegexError(regex::Error),
}

impl From<std::io::Error> for NodeError {
    fn from(err: std::io::Error) -> Self {
        NodeError::IoError(err)
    }
}

impl From<regex::Error> for NodeError {
    fn from(err: regex::Error) -> Self {
        NodeError::RegexError(err)
    }
}