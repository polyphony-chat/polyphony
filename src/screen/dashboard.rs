use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use iced::Element;

pub struct Dashboard;

impl Dashboard {
    pub fn view<'a>(
        &'a self,
        instances: HashMap<UrlBundle, Instance>,
        users: HashMap<(UrlBundle, String, u16), ChorusUser>,
    ) -> Element<'a, crate::Message> {
        todo!()
    }
}
