#[cfg(test)]
mod unit_tests;
use iced::{Application, Command, Column, Element, Text};
use std::default::Default;

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
