use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    MissingNameArg,
    NoIpAddrFound,
    IpTypeMismatch,
    IoError(IoError),
    NonTextMessageReceived,
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

