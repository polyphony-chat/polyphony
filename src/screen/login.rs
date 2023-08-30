use iced::widget::{column, text};
use iced::{Command, Element};

use crate::Client;

pub struct Login;

impl Login {
    pub fn view<'a>(&'a self) -> Element<'a, crate::Message> {
        let text1 = text("mememe");
        let text2 = text("lalala");
        column!(text1, text2).into()
    }
}
