// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::instance::{ChorusUser, Instance};
use chorus::UrlBundle;
use hashbrown::HashMap;
use leptos::RwSignal;

use crate::GlobalIdentifier;

#[derive(Clone, Debug, Default)]
pub(crate) struct ChorusStore {
    pub(crate) instances: RwSignal<HashMap<UrlBundle, Instance>>,
    pub(crate) users: RwSignal<HashMap<GlobalIdentifier, ChorusUser>>,
    pub(crate) current_user: RwSignal<Option<GlobalIdentifier>>,
}
