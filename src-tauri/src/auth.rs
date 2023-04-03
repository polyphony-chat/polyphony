pub mod auth {

    use crate::backend::URLBundle;
    use reqwest::{Client, Error, RequestBuilder, Response};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegisterParams {
        // TODO: Do all of these attrs *have* to be pub, or is there a better way?
        pub email: String,
        pub password: String,
        pub username: String,
        pub consent: bool,
        pub fingerprint: String,
        pub invite: Option<String>,
        pub date_of_birth: String,
        pub gift_code_sku_id: Option<String>,
        pub captcha_key: Option<String>,
        pub promotional_email_opt_in: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginParams {
        pub login: String,
        pub password: String,
        pub undelete: bool,
        pub captcha_key: Option<String>,
        pub login_source: Option<String>,
        pub gift_code_sku_id: Option<String>,
    }

    pub async fn register_spacebar(urls: &URLBundle, params: RegisterParams) -> String {
        let json: String = serde_json::to_string(&params).unwrap();
        let client: Client = Client::new();
        let request: RequestBuilder = client
            .post(urls.get_api().to_owned() + "/auth/register/")
            .body(json);
        let result: Result<Response, Error> = request.send().await;
        match result {
            Ok(result) => match result.text().await {
                Ok(body) => body,
                Err(error) => {
                    panic!("Something went wrong. {}", error);
                }
            },
            Err(error) => {
                panic!("An error occured while registering. {}", error)
            }
        }
    }

    pub async fn login_spacebar(urls: &URLBundle, params: LoginParams) -> String {
        let json: String = serde_json::to_string(&params).unwrap();
        let client: Client = Client::new();
        let request: RequestBuilder = client
            .post(urls.get_api().to_owned() + "/auth/login/")
            .body(json);
        let result: Result<Response, Error> = request.send().await;
        match result {
            Ok(result) => match result.text().await {
                Ok(body) => body,
                Err(error) => {
                    panic!("Something went wrong. {}", error);
                }
            },
            Err(error) => {
                panic!("An error occured while logging in. {}", error)
            }
        }
    }
}
