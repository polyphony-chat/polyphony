pub mod classes {
    pub enum InstanceType {
        Fosscord,
        Discord,
    }

    trait Backend {
        // The backend trait will define all needed functions/behaviour for the client to
        // communicate with the backend. This will be used to abstract away the backend
    }

    struct BackendConnection<B: Backend> {}

    struct Instance {
        name: String,
        url: String,
        instance: InstanceType,
        connection: BackendConnection,
    }

    struct User {
        id: u64,
        username: String,
        discriminator: String,
        avatar: String,
        bot: bool,
        system: bool,
        mfa_enabled: bool,
        banner: String,
        accent_color: u32,
        locale: String,
        verified: bool,
        email: String,
        flags: u64,
        premium_type: u8,
        public_flags: u64,
        instance: Instance,
    }
}
