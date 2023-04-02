pub mod spacebar_backend {
    use crate::backend::Backend;
    use crate::backend::URLBundle;
    use reqwest::Client;
    use serenity::client::ClientBuilder;
    use serenity::model::gateway::GatewayIntents;
    use std::sync::{Arc, Mutex};

    pub struct SpacebarBackend {
        urls: URLBundle,
        pub http_client: Client,
        pub serenity_client: ClientBuilder,
    }

    #[async_trait::async_trait]
    impl Backend for SpacebarBackend {
        fn new(token: String, urls: URLBundle) -> Self {
            let http_client: Client = Client::new();
            let intents = GatewayIntents::privileged().union(GatewayIntents::non_privileged());
            let serenity_client: ClientBuilder = ClientBuilder::new(
                token,
                intents,
                Arc::new(Mutex::new(String::from(urls.get_api().to_owned()))),
                Arc::new(Mutex::new(String::from(urls.get_cdn().to_owned()))),
                Arc::new(Mutex::new(String::from(urls.get_wss().to_owned()))),
            );
            SpacebarBackend {
                urls,
                http_client,
                serenity_client,
            }
        }

        fn get_instance_urls(&self) -> &URLBundle {
            &self.urls
        }

        async fn check_health(self) -> bool {
            let resp = reqwest::get(self.urls.get_api().to_owned() + "/api/ping").await;
            match resp {
                Ok(resp) => {
                    if resp.status() == 200 {
                        return true;
                    } else {
                        return false;
                    }
                }
                Err(_) => {
                    return false;
                }
            }
        }
    }
}
