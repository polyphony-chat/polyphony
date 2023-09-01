use crate::{message, Message};
use iced::widget::{button, column, text, text_input};
use iced::{Element, Renderer};

#[derive(Debug, Default)]
pub struct Welcome {
    pub url_input: String,
    pub username_input: String,
    pub password_input: String,
    pub error: String,
    pub login_result: String,
}

impl Welcome {
    pub fn view(&'_ self) -> Element<'_, Message> {
        let url_input = text_input::<Message, Renderer>("URL", &self.url_input)
            .padding(15)
            .on_input(|input: String| message::Welcome::UrlChanged(input).into());
        let username_input = text_input::<Message, Renderer>("Username", &self.username_input)
            .padding(15)
            .on_input(|input: String| message::Welcome::UsernameChanged(input).into());
        let password_input = text_input::<Message, Renderer>("Password", &self.password_input)
            .padding(15)
            .on_input(|input: String| message::Welcome::PasswordChanged(input).into());
        let login =
            button::<Message, Renderer>("Login").on_press(message::Welcome::LoginPressed.into());
        let error = text(self.error.clone());
        let login_result = text(self.login_result.clone());
        column!(
            url_input,
            username_input,
            password_input,
            login,
            error,
            login_result
        )
        .into()
    }
}
