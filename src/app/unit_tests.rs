use crate::{
    Error,
    Result
};
use super::*;
use std::net::{TcpListener, Ipv4Addr};

pub enum Message {
    Ping,
    Hello
}
//
//fn ping(mut socket: tungstenite::WebSocket<Stream::Plain(String)>) -> Result<String> {
//    socket.write_message(Message::Text("ping".into()))?;
//    match socket.read_message()? {
//        Message::Text(s) => Ok(s),
//        _ => Err(Error::NonTextMessageReceived),
//    }
//}

// TODO - convert the Stream to Tls encryption

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    let expected_socket = SocketAddr::from(([127,0,0,1], 4444));
    let sut = App::start;

    // When
    let app = sut()?;

    // Then
    assert_eq!(app.local_socket, expected_socket);
    Ok(())
}

//#[test]
//fn ping__live_socket_replies_with_pong() {
//    // Given
//    let app = App::new();
//    let app_socket = app.socket();
//
//    // When
//    let res = ping(app_socket);
//
//    // Then
//    assert_eq!(res, Ok("pong".to_string()));
//}

