use chorus::instance::Instance;
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use iced::futures::executor::block_on;
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

#[tokio::main]
async fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    username: String,
    discrim: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            username: "Not logged in".to_string(),
            discrim: "#0000".to_string(),
        }
    }

    fn title(&self) -> String {
        String::from("client")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                block_on(async {
                    let urls = UrlBundle::new(
                        "http://localhost:3001/api".to_string(),
                        "ws://localhost:3001".to_string(),
                        "http://localhost:3001".to_string(),
                    );
                    let mut instance = Instance::new(urls, false).await.unwrap();
                    let user = instance
                        .register_account(&RegisterSchema {
                            username: "Flori on Polyphony".to_string(),
                            consent: true,
                            date_of_birth: Some("1999-01-01".to_string()),
                            ..Default::default()
                        })
                        .await
                        .unwrap();
                    self.username = user.object.read().unwrap().username.clone();
                    self.discrim =
                        "#".to_string() + user.object.read().unwrap().discriminator.as_str();
                });
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            text(self.username.clone() + &self.discrim.clone()).size(20),
            button("Create account").on_press(Message::IncrementPressed),
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
