use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{message, Client, GlobalIdentifier, Message};
use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::widget::{button, column, text};
use iced::{Element, Renderer};

#[derive(Debug, Default, Clone)]
pub struct Dashboard {
    current_user: Option<ChorusUser>,
    users: Arc<RwLock<HashMap<GlobalIdentifier, ChorusUser>>>,
    instances: Arc<RwLock<HashMap<UrlBundle, Instance>>>,
}

impl Dashboard {
    pub fn view(&'_ self, client: &Client) -> Element<crate::Message> {
        let users = text::<Renderer>(format!(
            "Logged in as {:?}",
            self.users
                .read()
                .unwrap()
                .values()
                .map(|x| x.object.read().unwrap().username.clone())
                .collect::<Vec<String>>()
        ));
        let another_login = button::<Message, Renderer>("Login as another user")
            .on_press(message::Dashboard::ToLogin.into());
        column!(text("Welcome to the Dashboard."), users, another_login).into()
    }

    pub fn get_cache(client: &Client) -> Self {
        if let Some(cached) = client.cache.dashboard.clone() {
            cached
        } else {
            Dashboard {
                instances: client.instances.clone(),
                users: client.users.clone(),
                ..Default::default()
            }
        }
    }
}
