pub mod spacebar_backend {
    use crate::backend::Backend;
    use crate::backend::URLBundle;
    use reqwest::Client;

    pub struct SpacebarBackend {
        urls: URLBundle,
        pub http_client: Client,
    }

    #[async_trait::async_trait]
    impl Backend for SpacebarBackend {
        async fn new(token: String, urls: URLBundle) -> Self {
            let http_client: Client = Client::new();
            SpacebarBackend { urls, http_client }
        }

        fn get_instance_urls(&self) -> &URLBundle {
            &self.urls
        }

        async fn check_health(urls: &URLBundle) -> bool {
            let resp = reqwest::get(urls.get_api().to_owned() + "ping").await;
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
