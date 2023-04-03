pub mod backend {
    use url::{ParseError, Url};

    #[derive(Clone, Default)]
    pub struct URLBundle {
        pub api: String,
        pub wss: String,
        pub cdn: String,
    }

    impl URLBundle {
        pub fn new(api: String, wss: String, cdn: String) -> Self {
            Self {
                api: URLBundle::parse_url(api),
                wss: URLBundle::parse_url(wss),
                cdn: URLBundle::parse_url(cdn),
            }
        }

        /// parse(url: String) parses a URL using the Url library and formats it in a standardized
        /// way: http(s)://subdomain.domain.toplevel
        pub fn parse_url(url: String) -> String {
            let url = match Url::parse(&url) {
                Ok(url) => url,
                Err(ParseError::RelativeUrlWithoutBase) => {
                    let url = format!("http://{}", url);
                    Url::parse(&url).unwrap()
                }
                Err(_) => panic!("Invalid URL"),
            };
            url.to_string()
        }

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
