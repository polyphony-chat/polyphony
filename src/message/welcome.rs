use iced::Command;

use crate::Message;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Welcome {
    UrlChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
}

impl From<Welcome> for Message {
    fn from(welcome: Welcome) -> Self {
        Message::Welcome(welcome)
    }
}

impl Welcome {
    pub fn update(state: &mut crate::screen::Welcome, message: Self) -> iced::Command<Message> {
        match message {
            Self::UrlChanged(value) => {
                state.url_input = value;
                Command::none()
            }
            Self::UsernameChanged(value) => {
                state.username_input = value;
                Command::none()
            }
            Self::PasswordChanged(value) => {
                state.password_input = value;
                Command::none()
            }
            _ => todo!(),
        }
    }
}
