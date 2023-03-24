pub mod backend {

    #[async_trait::async_trait]
    pub trait Backend {
        /// The backend trait will define all needed functions/behaviour for the client to
        /// communicate with the backend. This will be used to abstract away the backend

        /// The backend object.
        fn new(instance_url: String) -> Self;
        async fn check_health(&self) -> bool;
    }

    pub struct FosscordBackend {
        instance_url: String,
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
    }

    /*  impl Backend for DiscordBackend {
        fn new(instance_url: String) -> Self {
            DiscordBackend {
                instance_url: instance_url,
            }
        }
    } */
}
