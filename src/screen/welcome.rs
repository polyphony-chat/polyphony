use std::collections::HashMap;

use crate::message;
use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::widget::{column, text_input};
use iced::{Element, Renderer};

#[derive(Debug, Default)]
pub struct Welcome {
    pub url_input: String,
    pub username_input: String,
    pub password_input: String,
}

impl Welcome {
    pub fn view<'a>(&'a self) -> Element<'a, crate::Message> {
        let url_input: iced::widget::TextInput<'_, crate::Message, Renderer> =
            text_input("Instance Base URL", &self.url_input).on_input();
        todo!()
    }
}
