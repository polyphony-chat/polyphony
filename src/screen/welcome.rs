use crate::{message, Message};
use iced::widget::{column, text_input};
use iced::{Element, Renderer};

#[derive(Debug, Default)]
pub struct Welcome {
    pub url_input: String,
    pub username_input: String,
    pub password_input: String,
}

impl Welcome {
    pub fn view(&'_ self) -> Element<'_, Message> {
        let url_input = text_input::<Message, Renderer>("URL", &self.url_input)
            .padding(15)
            .on_input(|input: String| message::Welcome::UrlChanged(input).into());
        let username_input = text_input::<Message, Renderer>("Username", &self.username_input)
            .padding(15)
            .on_input(|input: String| message::Welcome::UsernameChanged(input).into());
        let redacted_password = {
            let mut stars = String::new();
            for _ in 0..self.password_input.len() {
                stars += "*"
            }
            stars
        };
        let password_input = text_input::<Message, Renderer>("Password", &redacted_password)
            .padding(15)
            .on_input(|input: String| message::Welcome::PasswordChanged(input).into());
        column!(url_input, username_input, password_input).into()
    }
}
