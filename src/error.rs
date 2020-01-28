use std::io::Error as IoError;
use ws::Error as WsError;
#[derive(Debug)]
pub enum Error {
    MissingNameArg,
    NoIpAddrFound,
    IpTypeMismatch,
    IoError(IoError),
    NonTextMessageReceived,
    WebSocket(WsError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Error::IoError(err)
    }
}

impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Error::WebSocket(err)
    }
}
