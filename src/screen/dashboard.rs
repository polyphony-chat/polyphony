use std::collections::HashMap;

use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::Element;

pub struct Dashboard;

impl Dashboard {
    pub fn view(
        &'_ self,
        instances: HashMap<UrlBundle, Instance>,
        users: HashMap<(UrlBundle, String, u16), ChorusUser>,
    ) -> Element<crate::Message> {
        todo!("{}", format!("TODO {:?} {:?}", instances, users))
    }
}
