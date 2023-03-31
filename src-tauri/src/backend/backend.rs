pub mod backend {
    use crate::auth::auth::{LoginParams, RegisterParams};

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
}
