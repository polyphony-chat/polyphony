pub mod classes {
    use crate::{backend::backend::Backend, instance::instance::Instance};

    struct User<B>
    where
        B: Backend,
    {
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
        instance: Instance<B>,
    }
}
