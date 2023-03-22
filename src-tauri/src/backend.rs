pub mod backend {

    trait Backend {
        // The backend trait will define all needed functions/behaviour for the client to
        // communicate with the backend. This will be used to abstract away the backend

        /// The backend object.
        fn new() -> Self;
    }
}
