use chorus::instance::Instance;
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Sandbox, Settings};

#[tokio::main]
async fn main() -> iced::Result {
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
    Counter::run(Settings::default())
}

struct Counter {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
