use std::process::Command;
use iced::{Application, Command, Column, Text};
use std::default::Default;

#[derive(Default)]
pub(crate) struct ChatWindow {
}

#[derive(Debug, Clone, Copy)]
enum Message {
}

impl Application for ChatWindow {
    type Message = Message;

    fn new() -> (Self) {
        (Self::default())
    }

    fn title(&self) -> String {
        String::from("Chat App")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        Command::none()
    }

    fn view (&mut self) -> Column<Message> {
        Column::new()
            .push(
                Text::new(Text::new("Welcome to the Chat App")).size(50),
            )
    }
}
