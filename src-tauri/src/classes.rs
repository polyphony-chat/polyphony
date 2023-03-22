use backend::Backend;

pub mod classes {
    pub enum InstanceType {
        Fosscord,
        Discord,
    }

    struct BackendConnection<B: backend::Backend> {}

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
