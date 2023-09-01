use std::collections::HashMap;

use crate::{message, Message};
use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::widget::{button, column, text};
use iced::{Element, Renderer};

pub struct Dashboard;

impl Dashboard {
    pub fn view(
        &'_ self,
        instances: HashMap<UrlBundle, Instance>,
        users: HashMap<(UrlBundle, String, u16), ChorusUser>,
    ) -> Element<crate::Message> {
        let users = text::<Renderer>(format!(
            "Logged in as {:?}",
            users
                .values()
                .map(|x| x.object.read().unwrap().username.clone())
                .collect::<Vec<String>>()
        ));
        let another_login = button::<Message, Renderer>("Login as another user")
            .on_press(message::Dashboard::ToLogin.into());
        column!(text("Welcome to the Dashboard."), users, another_login).into()
    }
}
