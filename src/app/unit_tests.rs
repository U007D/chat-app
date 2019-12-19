use crate::MyError;
use super::*;
//use std::net::{TcpListener, TcpStream};
use crate::Result;

pub enum Message {
    Ping,
    Hello
}

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    let expected_addr = Ipv4Addr::new(127,0,0,1);
    let app = App::new();

    // When
    let res = app.start()?;

    // Then
    assert_eq!(res, expected_addr);
    Ok(())
}

//#[test]
//fn start__app_responds_to_ping() -> Result<String> {
//    // Given
//    let app = App::new();
//
//    // When
//    let address = app.start()?;
//    let msg = Message::Ping;
//    // Test: communicate with app via a tcp stream by sending it the message
//    // To come: starting tcp listener is in the impl of the app
//
//    // Then
//    assert_eq!(res, "OK");
//}