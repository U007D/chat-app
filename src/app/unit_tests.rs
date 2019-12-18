use super::*;
use std::net::{TcpListener, TcpStream};
pub enum MyError {}

pub enum Message {
    Ping,
    Hello
}

pub type Result<T, E = MyError> = std::result::Result<T, E>;

#[test]
fn start__app_responds_to_ping() -> Result<String> {
    // Given
    let app = App::new();

    // When
    let address = app.start()?;
    let msg = Message::Ping;
    // Test: communicate with app via a tcp stream by sending it the message
    // To come: starting tcp listener is in the impl of the app

    // Then
    assert_eq!(res, "OK");
}