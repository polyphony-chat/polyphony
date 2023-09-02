mod message;
mod screen;

use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

use chorus::instance::{ChorusUser, Instance};
use chorus::types::Snowflake;
use chorus::UrlBundle;
use iced::widget::container;
use iced::{Application, Command, Element, Length, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

/// (URLs, User-ID)
pub type GlobalIdentifier = (UrlBundle, Snowflake);

pub struct Client {
    pub instances: Arc<RwLock<HashMap<UrlBundle, Instance>>>,
    pub users: Arc<RwLock<HashMap<GlobalIdentifier, ChorusUser>>>,
    pub screen: Screen,
    pub cache: Cache,
}

#[derive(Debug, Default)]
pub struct Cache {
    pub dashboard: Option<screen::Dashboard>,
    pub messages: HashMap<GlobalIdentifier, Vec<Message>>,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            instances: Default::default(),
            users: Default::default(),
            screen: Screen::Welcome(screen::Welcome::default()),
            cache: Cache::default(),
        }
    }
}

pub enum Screen {
    Login(screen::Login),
    Dashboard(screen::Dashboard),
    Welcome(screen::Welcome),
}

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Login(_) => write!(f, "Login"),
            Self::Dashboard(_) => write!(f, "Dashboard"),
            Self::Welcome(_) => write!(f, "Welcome"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Login(message::Login),
    Dashboard(message::Dashboard),
    Welcome(message::Welcome),
}

impl Application for Client {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Client::default(), Command::none())
    }

    fn title(&self) -> String {
        format!("Client - {}", self.screen)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Welcome(message) => {
                let Screen::Welcome(_) = &mut self.screen else {
                    return Command::none();
                };
                message::Welcome::update(self, message)
            }
            Message::Dashboard(message) => {
                let Screen::Dashboard(_) = &mut self.screen else {
                    return Command::none();
                };
                message::Dashboard::update(self, message)
            }
            _ => todo!("Implement this updatemessage in main.rs!"),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match &self.screen {
            Screen::Login(login) => login.view(),
            Screen::Dashboard(dash) => dash.view(self),
            Screen::Welcome(welcome) => welcome.view(),
        };
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
