use std::io::Error;

#[derive(Debug)]
pub enum Error {
    MissingNameArg,
    NoIpAddrFound,
    IpTypeMismatch,
    IoError(std::io::Error),
    NonTextMessageReceived,
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}