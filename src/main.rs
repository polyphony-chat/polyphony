use chorus::instance::{ChorusUser, Instance};
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use iced::futures::executor::block_on;
use iced::widget::{button, column, text};
use iced::{Alignment, Application, Command, Element, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

enum Client {
    Disconnected,
    Connected(Polyphony),
}

#[derive(Debug, Clone, Copy)]
enum Message {
    GuildJoined,
    MessageReceived,
    Connect,
}

impl Application for Client {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let urls = UrlBundle::new(
            "http://localhost:3001/api".to_string(),
            "ws://localhost:3001/".to_string(),
            "http://localhost:3001/".to_string(),
        );
        let register = RegisterSchema {
            username: "userrrrrr".to_string(),
            password: Some("test".to_string()),
            consent: true,
            ..Default::default()
        };
        (
            Client::Disconnected,
            Command::perform(Polyphony::new(urls, register), |_| ()),
        )
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        todo!()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        todo!()
    }
}

#[derive(Debug)]
struct Polyphony {
    instance: Instance,
    user: ChorusUser,
}

impl Polyphony {
    async fn new(urls: UrlBundle, register: RegisterSchema) -> Self {
        let mut instance = Instance::new(urls, false).await.unwrap();
        let user = instance.register_account(&register).await.unwrap();
        Self { instance, user }
    }
}
