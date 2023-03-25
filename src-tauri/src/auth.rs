pub mod auth {
    use crate::backend::backend::Backend;
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

    pub struct LoginParams {
        login: String,
        password: String,
        undelete: bool,
        captcha_key: Option<String>,
        login_source: Option<String>,
        gift_code_sku_id: Option<String>,
    }

    pub async fn register(backend_object: &impl Backend, params: RegisterParams) -> String {
        return String::from("Hello"); //TODO: Implement this
    }

    pub async fn login(backend_object: &impl Backend, params: LoginParams) -> String {
        return String::from("Hello"); //TODO: Implement this
    }
}
