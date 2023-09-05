use std::sync::{Arc, RwLock};

use crate::{message, Client, Data, GlobalIdentifier, Message};
use async_trait::async_trait;
use chorus::gateway::Observer;
use chorus::instance::ChorusUser;
use chorus::types::{Guild, GuildCreate, GuildCreateDataOption, GuildDelete, GuildUpdate};
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
        if let Some(cached) = client.data.read().unwrap().dashboard.clone() {
            cached
        } else {
            Dashboard {
                data: client.data.clone(),
                ..Default::default()
            }
        }
    }
}

// TODO: i wrongly thought that i couldn't make all of these into just one struct, because i thought that any given struct can only have functions with unique names. however, if it's trait impls, this isn't the case, because the caller can specify which trait to call a certain function from.

#[derive(Debug)]
pub struct GuildUpdatesObserver {
    pub data: Arc<RwLock<Data>>,
}

#[async_trait]
impl Observer<GuildUpdate> for GuildUpdatesObserver {
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
                Some(data.guild.clone()),
            ),
            message::dashboard::GuildUpdateType::Update,
        );
    }
}

#[async_trait]
impl Observer<GuildDelete> for GuildUpdatesObserver {
    async fn update(&self, data: &GuildDelete) {
        let read = self.data.read().unwrap();
        let _ = message::Dashboard::ReceivedGuildUpdate(
            (
                (
                    read.url_to_bundle.get(&data.source_url).unwrap().clone(),
                    data.guild.id,
                ),
                None,
            ),
            message::dashboard::GuildUpdateType::Remove,
        );
    }
}

#[async_trait]
impl Observer<GuildCreate> for GuildUpdatesObserver {
    async fn update(&self, data: &GuildCreate) {
        match data.d {
            GuildCreateDataOption::UnavailableGuild(_) => todo!(),
            GuildCreateDataOption::Guild(_) => todo!(),
        }
    }
}
