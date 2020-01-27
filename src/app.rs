#[cfg(test)]
mod unit_tests;

use std::thread::{self, JoinHandle, spawn};
use std::default::Default;
use std::net::{TcpListener, Ipv4Addr, TcpStream};

use iced::{Application, Command, Column, Element, Text};
use get_if_addrs::{get_if_addrs, IfAddr};
use tungstenite::{
    self,
    stream::Stream,
    WebSocket,
    accept_hdr,
    handshake::server::{Request, Response}
};

use crate::Result;
use crate::Error;

pub struct App {
    socket: Option<tungstenite::WebSocket<Stream::Plain(String)>>,
    server: Option<TcpListener>,
    serverHandler: Option<JoinHandle<()>>
}

impl App {
    pub fn new() -> Self {
        Self{ socket: None, server: None, serverHandler: None }
    }

    pub fn start(&mut self) -> Result<Ipv4Addr> {
        let addrs = get_if_addrs()?;
        let default_addr = addrs
            .into_iter()
            .nth(0)
            .map_or_else(||Err(Error::NoIpAddrFound), |intrfc | {
                match intrfc.addr {
                    IfAddr::V4(addr) => Ok(addr.ip),
                    IfAddr::V6(addr) => Err(Error::IpTypeMismatch)
                }
            });
        self.start_server(default_addr)?;
        self.serverHandler = Some(thread::spawn(|| {
            self.listen_for_connections();
        }));
        default_addr
    }

    fn start_server(&mut self, address: String) -> Result<()> {
        let server = TcpListener::bind(address);
        let server  = match server {
            Ok(server) => server,
            Err(s) => return Err(Error::from(s))
        };
        self.server = Some(server);
        Ok(())
    }

    fn listen_for_connections(&mut self) -> Result<()> {
        if let None = self.server {
            return Err(Error::NoIpAddrFound);
        }

        for stream in self.server.unwrap().incoming() {
            self.accept_handshake(stream.unwrap());
        }

        Ok(())
    }

    fn new_connection(req: &Request, mut response: Response) -> Result<Response> {
        println!("Received a new ws handshake");
        println!("The request's path is: {}", req.uri().path());
        println!("The request's headers are:");
        for (ref header, _value) in req.headers() {
            println!("* {}", header);
        }
        Ok(response)
    }

    fn accept_handshake(&mut self, stream: TcpStream) -> Result<()> {
        self.socket = Some(accept_hdr(stream, self.new_connection))
    }

    fn monitor_socket(mut socket: tungstenite::WebSocket<Stream::Plain(String)>) {
        loop {
            let msg = socket.read_message()
        }
    }
}

#[derive(Default)]
pub(crate) struct ChatWindow {
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
}

impl Application for ChatWindow {
    type Message = Message;

    fn new() -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Chat App")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view (&mut self) -> Element<Message> {
        Column::new()
            .push(
                Text::new("Welcome to the Chat App").size(50)
            ).into()
    }
}
