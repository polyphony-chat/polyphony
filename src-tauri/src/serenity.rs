pub mod serenity {
    use serenity::{client::Client, prelude::GatewayIntents};
    use std::sync::{Arc, Mutex};

    use crate::backend::URLBundle;

    /// Construct a new serenity client, await it and handle errors which might occur.
    pub async fn new(token: String, intents: GatewayIntents, urls: &URLBundle) -> Client {
        let client_result = Client::builder(
            token,
            intents,
            Arc::new(Mutex::new(urls.get_api().to_owned())),
            Arc::new(Mutex::new(urls.get_cdn().to_owned())),
            Arc::new(Mutex::new(urls.get_wss().to_owned())),
        )
        .await;

        match client_result {
            Ok(client) => client,
            Err(error) => {
                panic!("An error occured: {}", error)
            }
        }
    }
}
