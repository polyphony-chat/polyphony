pub mod auth {
    use crate::backend::backend::{Backend, FosscordBackend};
    use reqwest::{Client, Error, RequestBuilder, Response};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct RegisterParams {
        email: String,
        password: String,
        username: String,
        consent: bool,
        fingerprint: String,
        invite: Option<String>,
        date_of_birth: String,
        gift_code_sku_id: Option<String>,
        captcha_key: Option<String>,
        promotional_email_opt_in: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct LoginParams {
        login: String,
        password: String,
        undelete: bool,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    }

    pub async fn register_fosscord(
        backend_object: &FosscordBackend,
        params: RegisterParams,
    ) -> String {
        let json: String = serde_json::to_string(&params).unwrap();
        let client: &Client = &backend_object.http_client;
        let request: RequestBuilder = client
            .post(backend_object.instance_url.clone() + "/api/register/")
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

    pub async fn login_fosscord(backend_object: &FosscordBackend, params: LoginParams) -> String {
        return String::from("Hello"); //TODO: Implement this
    }
}
