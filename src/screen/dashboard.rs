use std::collections::HashMap;
use std::sync::{Arc, RwLock, Weak};

use crate::{message, Client, Data, GlobalIdentifier, Message};
use async_trait::async_trait;
use chorus::gateway::Observer;
use chorus::instance::{ChorusUser, Instance};
use chorus::types::{Guild, GuildUpdate};
use chorus::UrlBundle;
use iced::widget::{button, column, row, text};
use iced::{Element, Renderer};

#[derive(Debug, Default, Clone)]
pub struct Dashboard {
    pub current_user: Option<ChorusUser>,
    pub data: Arc<RwLock<Data>>,
    pub guilds: Vec<(GlobalIdentifier, Guild)>,
}

impl Dashboard {
    pub fn view(&'_ self, client: &Client) -> Element<crate::Message> {
        let users = text::<Renderer>(format!(
            "Logged in as {:?}",
            self.data
                .read()
                .unwrap()
                .users
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
                data: client.data.clone(),
                ..Default::default()
            }
        }
    }
}

#[derive(Debug)]
struct GuildAddObserver {
    client: Arc<RwLock<Client>>,
}
#[derive(Debug)]
pub struct GuildUpdateObserver {
    pub data: Arc<RwLock<Data>>,
}
#[derive(Debug)]
struct GuildRemoveObserver {
    client: Weak<RwLock<Client>>,
}

#[async_trait]
impl Observer<GuildUpdate> for GuildUpdateObserver {
    async fn update(&self, data: &GuildUpdate) {
        let _ = message::Dashboard::ReceivedGuildUpdate(
            (
                (
                    self.data
                        .read()
                        .unwrap()
                        .url_to_bundle
                        .get(&data.source_url)
                        .unwrap()
                        .clone(),
                    data.guild.id,
                ),
                data.guild.clone(),
            ),
            message::dashboard::GuildUpdateType::Update,
        );
    }
}
