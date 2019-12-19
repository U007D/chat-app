#[cfg(test)]
mod unit_tests;
use get_if_addrs::{get_if_addrs, IfAddr};
use iced::{Application, Command, Column, Element, Text};
use std::default::Default;
use std::net::Ipv4Addr;
use crate::Result;
use crate::MyError;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self{}
    }

    pub fn start(&self) -> Result<Ipv4Addr> {
        let addrs = get_if_addrs()?;
        addrs
            .into_iter()
            .nth(0)
            .map_or_else(||Err(MyError::NoIpAddrFound), | intrfc | {
                match intrfc.addr {
                    IfAddr::V4(addr) => Ok(addr.ip),
                    IfAddr::V6(addr) => Err(MyError::IpTypeMismatch)
                }
            })
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
