pub mod backend {
    use crate::auth::auth;
    use crate::auth::auth::{LoginParams, RegisterParams};
    use reqwest::Client;

    #[async_trait::async_trait]
    pub trait Backend {
        /// The backend trait will define all needed functions/behaviour for the client to
        /// communicate with the backend. This will be used to abstract away the backend

        /// The backend object.
        fn new(instance_url: String) -> Self;
        async fn check_health(&self) -> bool;
        async fn register(&self, params: RegisterParams) -> String;
        async fn login(&self, params: LoginParams) -> String;
        fn get_instance_url(&self) -> String;
    }

    pub struct SpacebarBackend {
        pub instance_url: String,
        pub http_client: Client,
    }

    /*     pub struct DiscordBackend {
           instance_url: String,
       }
    */
    #[async_trait::async_trait]
    impl Backend for SpacebarBackend {
        fn new(instance_url: String) -> Self {
            let client: Client = Client::new();
            SpacebarBackend {
                instance_url: instance_url,
                http_client: client,
            }
        }

        fn get_instance_url(&self) -> String {
            self.instance_url.clone()
        }

        async fn check_health(&self) -> bool {
            let url: String = String::from(&self.instance_url.clone());
            let resp = reqwest::get(url + "/api/ping").await;
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

        async fn register(&self, params: RegisterParams) -> String {
            auth::register_spacebar(self, params).await
        }

        async fn login(&self, params: LoginParams) -> String {
            auth::login_spacebar(self, params).await
        }
    }
}
