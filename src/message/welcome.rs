use crate::screen;
use chorus::errors::ChorusError;
use chorus::instance::{ChorusUser, Instance};
use chorus::types::LoginSchema;
use chorus::UrlBundle;
use iced::Command;

use crate::{Client, Message, Screen};

#[derive(Debug, Clone)]
pub enum Welcome {
    UrlChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    LoginPressed,
    InstanceCreateResultGotten(Result<Instance, ChorusError>),
    LoginRequestDone(Result<ChorusUser, ChorusError>),
}

impl From<Welcome> for Message {
    fn from(welcome: Welcome) -> Self {
        Message::Welcome(welcome)
    }
}

impl Welcome {
    pub fn update(state: &mut Client, message: Self) -> iced::Command<Message> {
        let Screen::Welcome(welcome) = &mut state.screen else {
            return Command::none();
        };
        match message {
            Self::UrlChanged(value) => {
                welcome.url_input = value;
                Command::none()
            }
            Self::UsernameChanged(value) => {
                welcome.username_input = value;
                Command::none()
            }
            Self::PasswordChanged(value) => {
                welcome.password_input = value;
                Command::none()
            }
            Self::LoginPressed => {
                let url_bundle = UrlBundle::new(
                    format!("http://{}/api", welcome.url_input),
                    format!("ws://{}", welcome.url_input),
                    format!("http://{}", welcome.url_input),
                );
                Command::perform(Instance::new(url_bundle, true), |result| {
                    Welcome::InstanceCreateResultGotten(result).into()
                })
            }
            Self::InstanceCreateResultGotten(result) => {
                if let Ok(result) = result {
                    let result_clone = result.clone();
                    state.instances.insert(result.urls.clone(), result.clone());
                    let login_schema: LoginSchema = LoginSchema {
                        login: welcome.username_input.clone(),
                        password: welcome.password_input.clone(),
                        ..Default::default()
                    };
                    let future = result_clone.login_account(login_schema);
                    return Command::perform(future, |loginresult| {
                        Message::from(Welcome::LoginRequestDone(loginresult))
                    });
                } else {
                    welcome.error = format!("Error: {:?}", result.err().unwrap());
                }
                Command::none()
            }
            Self::LoginRequestDone(result) => {
                if let Ok(result) = result {
                    state.users.insert(
                        (
                            result.belongs_to.read().unwrap().urls.clone(),
                            result.object.read().unwrap().id,
                        ),
                        result.clone(),
                    );
                    state.screen = Screen::Dashboard(screen::Dashboard);
                } else {
                    welcome.error = format!("Error: {:?}", result.err().unwrap())
                }
                Command::none()
            }
        }
    }
}
