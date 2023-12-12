use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use hashbrown::HashMap;
use leptos::RwSignal;

use crate::GlobalIdentifier;

#[derive(Clone, Debug, Default)]
pub(crate) struct ChorusStore {
    pub(crate) instances: RwSignal<HashMap<UrlBundle, Instance>>,
    pub(crate) users: RwSignal<HashMap<GlobalIdentifier, ChorusUser>>,
}
