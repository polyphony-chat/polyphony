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
        log::debug!("Dashboard: called view()");
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
        log::debug!("Cache: Called get_cache()");
        if let Some(cached) = client.data.read().unwrap().dashboard.clone() {
            log::debug!("Cache: Retrieving Dashboard from Cache");
            cached
        } else {
            log::debug!("Cache: No Dashboard in Cache. Creating new screen::Dashboard");
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
        log::debug!("Observer: Guild: Processing GuildUpdate");
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
        log::debug!("Observer: Guild: Processing GuildDelete");
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
        log::debug!("Observer: Guild: Processing GuildCreate");
        let read = self.data.read().unwrap();
        let _ = match &data.d {
            GuildCreateDataOption::UnavailableGuild(unavailable) => {
                log::debug!("Observer: Guild: GuildCreate: Guild is marked as Unavailable.");
                message::Dashboard::ReceivedGuildUpdate(
                    (
                        (
                            read.url_to_bundle.get(&data.source_url).unwrap().clone(),
                            unavailable.id,
                        ),
                        None,
                    ),
                    message::dashboard::GuildUpdateType::Add,
                )
            }
            GuildCreateDataOption::Guild(guild) => {
                log::debug!("Observer: Guild: GuildCreate: Guild is marked as Unavailable.");
                message::Dashboard::ReceivedGuildUpdate(
                    (
                        (
                            read.url_to_bundle.get(&data.source_url).unwrap().clone(),
                            guild.id,
                        ),
                        Some(guild.clone()),
                    ),
                    message::dashboard::GuildUpdateType::Add,
                )
            }
        };
    }
}
