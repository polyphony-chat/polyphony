pub mod backend {

    #[derive(Clone, Default)]
    pub struct URLBundle {
        api: String,
        wss: String,
        cdn: String,
    }

    impl URLBundle {
        pub fn get_api(&self) -> &str {
            &self.api
        }

        pub fn get_cdn(&self) -> &str {
            &self.cdn
        }

        pub fn get_wss(&self) -> &str {
            &self.wss
        }
    }
    #[async_trait::async_trait]
    pub trait Backend {
        /// The backend trait will define all needed functions/behaviour for the client to
        /// communicate with the backend. This will be used to abstract away the backend
        async fn check_health(self) -> bool;
        fn new(token: String, urls: URLBundle) -> Self;
        fn get_instance_urls(&self) -> &URLBundle;
    }
}
