pub mod backend {
    use crate::auth::auth;
    use crate::auth::auth::{LoginParams, RegisterParams};
    #[async_trait::async_trait]
    pub trait Backend {
        /// The backend trait will define all needed functions/behaviour for the client to
        /// communicate with the backend. This will be used to abstract away the backend

        /// The backend object.
        fn new(instance_url: String) -> Self;
        async fn check_health(&self) -> bool;
        async fn perform_register(&self, params: RegisterParams) -> String;
        async fn perform_login(&self, params: LoginParams) -> String;
    }

    pub struct FosscordBackend {
        instance_url: String,
        instance_type: String,
    }

    /*     pub struct DiscordBackend {
           instance_url: String,
       }
    */
    #[async_trait::async_trait]
    impl Backend for FosscordBackend {
        fn new(instance_url: String) -> Self {
            FosscordBackend {
                instance_url: instance_url,
                instance_type: String::from("fosscord"),
            }
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

        async fn perform_register(&self, params: RegisterParams) -> String {
            auth::register(self, params).await
        }

        async fn perform_login(&self, params: LoginParams) -> String {
            auth::login(self, params).await
        }
    }
}
