mod message;
mod screen;

use std::collections::HashMap;
use std::fmt::Display;

use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::widget::container;
use iced::{Application, Command, Element, Length, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

pub struct Client {
    pub instances: HashMap<UrlBundle, Instance>,
    pub users: HashMap<(UrlBundle, String, u16), ChorusUser>, // Urls, Username, Discrim
    pub screen: Screen,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            instances: Default::default(),
            users: Default::default(),
            screen: Screen::Welcome(screen::Welcome::default()),
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
            _ => todo!(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match &self.screen {
            Screen::Login(login) => login.view(),
            Screen::Dashboard(dash) => dash.view(self.instances.clone(), self.users.clone()),
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
