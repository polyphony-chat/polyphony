use std::collections::HashMap;

use chorus::instance::{ChorusUser, Instance};
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use iced::widget::{button, column, container, text};
use iced::{Application, Command, Element, Length, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

#[derive(Debug, Clone)]
struct InstanceUserBundle {
    pub instance: Instance,
    pub user: ChorusUser,
}

#[derive(Debug, Default)]
struct Client {
    pub instances: HashMap<UrlBundle, Instance>,
    pub users: HashMap<UrlBundle, ChorusUser>,
}

impl Client {
    pub fn is_connected(&self) -> bool {
        !self.instances.is_empty() && !self.users.is_empty()
    }
}

#[derive(Debug, Clone)]
enum Message {
    GuildJoined,
    MessageReceived,
    Connect((UrlBundle, RegisterSchema)),
    Connected(InstanceUserBundle),
    Disconnected,
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
        let subtitle;
        if self.instances.is_empty() {
            subtitle = "Disconnected"
        } else {
            subtitle = "Connected"
        }

        format!("Client - {}", subtitle)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Connect((urls, register)) => {
                Command::perform(InstanceUserBundle::new(urls, register), Message::Connected)
            }
            Message::Connected(polyphony) => {
                self.instances
                    .insert(polyphony.instance.urls.clone(), polyphony.instance);
                let urls = polyphony.user.belongs_to.read().unwrap().urls.clone();
                self.users.insert(urls, polyphony.user);
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self.is_connected() {
            true => {
                column!(text(format!("Connected to {:?}", self.instances)))
            }
            false => {
                let urls = UrlBundle::new(
                    "http://localhost:3001/api".to_string(),
                    "ws://localhost:3001/".to_string(),
                    "http://localhost:3001/".to_string(),
                );
                let register = RegisterSchema {
                    username: "userrrrrr".to_string(),
                    date_of_birth: Some("1999-01-01".to_string()),
                    consent: true,
                    ..Default::default()
                };
                column!(
                    text("Disconnected"),
                    button("Connect").on_press(Message::Connect((urls, register)))
                )
            }
        };
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

impl InstanceUserBundle {
    async fn new(urls: UrlBundle, register: RegisterSchema) -> Self {
        let mut instance = Instance::new(urls, false).await.unwrap();
        let user = instance.register_account(&register).await.unwrap();
        Self { instance, user }
    }
}
