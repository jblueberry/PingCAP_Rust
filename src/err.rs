use failure::Fail;

#[derive(Fail, Debug)]
pub enum KvsError {
    /// IO error.
    #[fail(display = "{}", _0)]
    Io(#[cause] std::io::Error),
    /// Serialization or deserialization error.
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),
    /// Removing non-existent key error.
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupted log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
    #[fail(display = "Unknown engine")]
    UnknownEngine,
    #[fail(display = "{}", _0)]
    IpAddr(#[cause] std::net::AddrParseError),
}

impl From<serde_json::Error> for KvsError {
    fn from(value: serde_json::Error) -> Self {
        Self::Serde(value)
    }
}

impl From<std::io::Error> for KvsError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<std::net::AddrParseError> for KvsError {
    fn from(value: std::net::AddrParseError) -> Self {
        Self::IpAddr(value)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;