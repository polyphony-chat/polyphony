use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chorus::instance::ChorusUser;
use chorus::types::Guild;
use iced::Command;

use crate::{screen, Client, GlobalIdentifier, Message, Screen};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Dashboard {
    ToLogin,
    Guilds(Vec<(GlobalIdentifier, Guild)>),
}

impl From<Dashboard> for Message {
    fn from(dashboard: Dashboard) -> Self {
        Message::Dashboard(dashboard)
    }
}

impl Dashboard {
    pub fn update(client: &mut Client, message: Self) -> iced::Command<Message> {
        let Screen::Dashboard(dash) = &mut client.screen else {
            return Command::none();
        };
        match message {
            Self::ToLogin => {
                client.cache.dashboard = Some(dash.clone());
                client.screen = Screen::Welcome(screen::Welcome::default())
            }
            Self::Guilds(result) => dash.guilds = result,
        }
        Command::none() // TODO
    }

    pub async fn fetch_guilds(
        users: Arc<RwLock<HashMap<GlobalIdentifier, ChorusUser>>>,
    ) -> Vec<(GlobalIdentifier, Guild)> {
        let mut users_lock = users.write().unwrap().clone();
        let mut return_vec: Vec<(GlobalIdentifier, chorus::types::Guild)> = Vec::new();
        for (user_identifier, user) in users_lock.iter_mut() {
            let user_guilds = user
                .get_guilds(Some(chorus::types::GetUserGuildSchema {
                    before: None,
                    after: None,
                    limit: Some(128),
                    with_counts: None,
                }))
                .await
                .unwrap();
            for item in user_guilds {
                return_vec.push((user_identifier.clone(), item));
            }
        }
        return_vec
    }
}
