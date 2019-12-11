mod app;
use iced::{Settings, Application};
//use crate::MyError::MissingNameArg;
//use std::env;
use app::ChatWindow;

type Result<T, E = MyError> = std::result::Result<T, E>;
#[derive(Debug)]
enum MyError {
    MissingNameArg,
}
//FYI to self impl std::error::Error for Error {} // implement trait for type, Error type is now impls std error trait

fn main() -> Result<()> {
//    println!("Hello, {:?}", env::args().nth(1).ok_or(MissingNameArg)?);
    ChatWindow::run(Settings::default());
    Ok(())
}
