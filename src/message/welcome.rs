use iced::Command;

use crate::Message;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Welcome {
    UrlChanged(String),
}

impl Welcome {
    pub fn update(state: &mut crate::screen::Welcome, message: Self) -> iced::Command<Message> {
        match message {
            Self::UrlChanged(value) => {
                state.url_input = value;
                Command::none()
            }
        }
    }
}
