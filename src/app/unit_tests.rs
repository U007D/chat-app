use crate::{
    Error,
    Result
};
use super::*;
use Ipv4Addr;
use std::net::TcpListener;
use tungstenite::{
    self,
    stream::Stream,
    WebSocket
};

pub enum Message {
    Ping,
    Hello
}

fn ping(mut socket: tungstenite::WebSocket<Stream::Plain(String)>) -> Result<String> {
    socket.write_message(Message::Text("Ping".into()))?;
    match socket.read_message()? {
        Message::Text(s) => Ok(s),
        _ => Err(Error::NonTextMessageReceived),
    }
}

// TODO - convert the Stream to Tls encryption

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

#[test]
fn ping__live_socket_replies_with_pong() {
    // Given
    let app = App::new();
    let app_socket = app.socket();

    // When
    let res = ping(app_socket);

    // Then
    assert_eq!(res, Ok("pong".to_string()));
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