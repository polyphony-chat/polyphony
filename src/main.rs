mod message;
mod screen;

use std::collections::HashMap;

use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::widget::{button, column, container, text};
use iced::{Application, Command, Element, Length, Settings};
use screen::Login;

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

struct Client {
    pub instances: HashMap<UrlBundle, Instance>,
    pub users: HashMap<UrlBundle, ChorusUser>,
    pub screen: Screen,
}

impl Default for Client {
    fn default() -> Self {
        Self {
            instances: Default::default(),
            users: Default::default(),
            screen: Screen::Login(Login),
        }
    }
}

pub enum Screen {
    Login(screen::Login),
    Dashboard(screen::Dashboard),
    Welcome(screen::Welcome),
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
        let subtitle = if self.instances.is_empty() {
            "Disconnected"
        } else {
            "Connected"
        };

        format!("Client - {}", subtitle)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match &self.screen {
            Screen::Login(login) => login.view(),
            Screen::Dashboard(dash) => dash.view(&self.instances, &self.users),
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
