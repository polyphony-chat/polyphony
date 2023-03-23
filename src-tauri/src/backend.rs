pub mod backend {

    pub trait Backend {
        // The backend trait will define all needed functions/behaviour for the client to
        // communicate with the backend. This will be used to abstract away the backend

        /// The backend object.
        fn new() -> Self;
    }

    pub struct FosscordBackend {}

    pub struct DiscordBackend {}

    impl Backend for FosscordBackend {
        fn new() -> Self {
            FosscordBackend {}
        }
    }

    impl Backend for DiscordBackend {
        fn new() -> Self {
            DiscordBackend {}
        }
    }
}
