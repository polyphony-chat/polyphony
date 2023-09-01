use iced::Command;

use crate::{screen, Client, Message, Screen};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dashboard {
    ToLogin,
}

impl From<Dashboard> for Message {
    fn from(dashboard: Dashboard) -> Self {
        Message::Dashboard(dashboard)
    }
}

impl Dashboard {
    pub fn update(state: &mut Client, message: Self) -> iced::Command<Message> {
        let Screen::Dashboard(dash) = &mut state.screen else {
            return Command::none();
        };
        match message {
            Self::ToLogin => state.screen = Screen::Welcome(screen::Welcome::default()),
        }
        Command::none() // TODO
    }
}
