use std::io::Error;

#[derive(Debug)]
pub enum MyError {
    MissingNameArg,
    NoIpAddrFound,
    IpTypeMismatch,
    IoError(std::io::Error)
}

impl From<std::io::Error> for MyError{
    fn from(err: std::io::Error) -> Self {
        MyError::IoError(err)
    }
}