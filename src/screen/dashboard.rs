use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::{message, Client, GlobalIdentifier, Message};
use chorus::instance::{ChorusUser, Instance};
use chorus::types::Guild;
use chorus::UrlBundle;
use iced::widget::{button, column, row, text};
use iced::{Element, Renderer};

#[derive(Debug, Default, Clone)]
pub struct Dashboard {
    pub current_user: Option<ChorusUser>,
    pub users: Arc<RwLock<HashMap<GlobalIdentifier, ChorusUser>>>,
    pub instances: Arc<RwLock<HashMap<UrlBundle, Instance>>>,
    pub guilds: Vec<(GlobalIdentifier, Guild)>,
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
        let mut guilds: iced::widget::Column<'_, Message, Renderer> = column!();
        for (_, guild) in self.guilds.iter() {
            guilds = guilds.push(text(guild.name.as_ref().unwrap().clone()));
        }
        row!(
            column!(text("Welcome to the Dashboard."), users, another_login),
            guilds
        )
        .into()
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
