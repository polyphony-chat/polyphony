use chorus::instance::{ChorusUser, Instance};
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use iced::futures::executor::block_on;
use iced::widget::{button, column, container, text};
use iced::{Alignment, Application, Command, Element, Length, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Client::run(Settings::default())
}

enum Client {
    Disconnected,
    Connected(Polyphony),
}

#[derive(Debug, Clone)]
enum Message {
    GuildJoined,
    MessageReceived,
    Connect((UrlBundle, RegisterSchema)),
    Connected(Polyphony),
}

impl Application for Client {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = iced::Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Client::Disconnected, Command::none())
    }

    fn title(&self) -> String {
        let subtitle = match self {
            Client::Disconnected => "Disconnected",
            Client::Connected(_) => "Connected",
        };

        format!("Client - {}", subtitle)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Connect((urls, register)) => {
                Command::perform(Polyphony::new(urls, register), Message::Connected)
            }
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let content = match self {
            Client::Connected(instance) => {
                column!(text(format!(
                    "Connected with token {}",
                    instance.user.token()
                )))
            }
            Client::Disconnected => {
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

#[derive(Debug)]
struct Polyphony {
    pub instance: Instance,
    pub user: ChorusUser,
}

impl Polyphony {
    async fn new(urls: UrlBundle, register: RegisterSchema) -> Self {
        let mut instance = Instance::new(urls, false).await.unwrap();
        let user = instance.register_account(&register).await.unwrap();
        Self { instance, user }
    }
}
