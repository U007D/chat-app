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

#[test]
fn start__app_starts() -> Result<()> {
    // Given
    let expected_socket = SocketAddr::from(([127,0,0,1], 4444));
    let sut = App::start;

    // When
    let actual_app = sut()?;

    // Then
    assert_eq!(actual_app.local_socket, expected_socket);
    Ok(())
}

// https://github.com/housleyjk/ws-rs/blob/master/examples/client.rs
#[test]
fn ping__live_socket_replies_with_pong() {
    // Given
    let expected_socket = SocketAddr::from(([127,0,0,1], 4444));
    let app = App::start()?;
    let listner_socket = app.local_socket;

    // When

    // Then
}

