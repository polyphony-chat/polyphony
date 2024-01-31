// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chorus::errors::ChorusResult;
use chorus::instance::{ChorusUser, Instance};
use chorus::types::RegisterSchema;
use chorus::UrlBundle;
use hashbrown::HashMap;
use leptos::*;
use log::*;

use crate::GlobalIdentifier;

#[component]
pub fn Register() -> impl IntoView {
    let (mail, set_mail) = create_signal(String::new());
    let (pass, set_pass) = create_signal(String::new());
    let (url, set_url) = create_signal(String::new());

    let submit = create_action(|input: &(String, String, String)| {
        let input = input.to_owned();
        async move { register(&input).await }
    });
    debug!("Rendering Register component");
    // TODO: Set an error if registering fails
    view! {
        <form on:submit=move |ev| {
            ev.prevent_default();
            let input = (url.get().to_string(), pass.get().to_string(), mail.get().to_string());
            submit.dispatch(input);
        }>
            <label for="mail">Email:</label>
            <input class="border-2 border-black text-black" type="email" id="mail" name="email" on:input=move |ev| {
                set_mail.set(event_target_value(&ev));
            } prop:value=mail/><br/>
            <label for="pass">Password:</label>
            <input class="border-2 border-black text-black" type="password" id="pass" name="pass" on:input=move |ev| {
                set_pass.set(event_target_value(&ev));
            } prop:value=pass/><br/>
            <label for="iurl">Instance URL:</label>
            <input class="border-2 border-black text-black" type="text" id="iurl" name="iurl" on:input=move |ev| {
                set_url.set(event_target_value(&ev));
            } prop:value=url/><br/>
            <button type="submit" id="submitbutton">Submit</button>
        </form>
    }
}

async fn register(input: &(String, String, String)) -> ChorusResult<()> {
    let register_schema = RegisterSchema {
        username: input.2.clone(),
        password: Some(input.1.clone()),
        consent: true,
        email: Some(input.2.clone()),
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };
    let instance_store = use_context::<RwSignal<HashMap<UrlBundle, Instance>>>().unwrap();
    let urls = UrlBundle::from_root_url(&input.0).await?;
    let mut instance = match instance_store.get_untracked().get(&urls) {
        Some(cached_instance) => {
            debug!("Got cached instance: {:#?}", cached_instance);
            cached_instance.clone()
        }
        None => {
            debug!("No cached instance found. Attempting lookup");
            debug!("Attempting to create instance from URLBundle {:#?}", &urls);
            let new_instance = Instance::new(urls.clone()).await.unwrap();
            debug!(
                "Got fresh instance: {:#?}; Inserting into cache",
                &new_instance
            );
            instance_store.update(|map| {
                map.insert(urls.clone(), new_instance.clone());
            });
            new_instance
        }
    };
    debug!(
        "Registering account on instance {}",
        instance.instance_info.instance_name
    );
    let account = instance.register_account(register_schema).await;
    instance_store.update(|map| {
        map.insert(urls.clone(), instance);
    });
    debug!("Got account with token {}", account.as_ref().unwrap().token);
    if let Ok(account) = account {
        let user_store = use_context::<RwSignal<HashMap<GlobalIdentifier, ChorusUser>>>().unwrap();
        let current_user = use_context::<RwSignal<Option<GlobalIdentifier>>>().unwrap();
        let id = account.object.read().unwrap().id;
        user_store.update(|map| {
            map.insert((urls.clone(), id), account);
        });
        current_user.set(Some((urls.clone(), id)));
        Ok(())
    } else {
        Err(account.unwrap_err())
    }
}
