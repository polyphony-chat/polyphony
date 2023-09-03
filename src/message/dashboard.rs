use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use chorus::instance::ChorusUser;
use chorus::types::Guild;
use iced::Command;

use crate::{screen, Client, GlobalIdentifier, Message, Screen};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Dashboard {
    ToLogin,
}

impl From<Dashboard> for Message {
    fn from(dashboard: Dashboard) -> Self {
        Message::Dashboard(dashboard)
    }
}

impl Dashboard {
    pub fn update(state: &mut Client, message: Self) -> iced::Command<Message> {
        let Screen::Dashboard(dash) = &mut state.screen else {
            return Command::none();
        };
        match message {
            Self::ToLogin => {
                state.cache.dashboard = Some(dash.clone());
                state.screen = Screen::Welcome(screen::Welcome::default())
            }
        }
        Command::none() // TODO
    }

    async fn fetch_guilds(
        users: Arc<RwLock<HashMap<GlobalIdentifier, ChorusUser>>>,
    ) -> Vec<(GlobalIdentifier, Guild)> {
        let mut users_lock = users.write().unwrap().clone();
        let mut return_vec: Vec<(GlobalIdentifier, chorus::types::Guild)> = Vec::new();
        for (user_identifier, user) in users_lock.iter_mut() {
            let user_guilds = user.get_guilds(None).await.unwrap();
            for item in user_guilds {
                return_vec.push((user_identifier.clone(), item));
            }
        }
        return_vec
    }
}
