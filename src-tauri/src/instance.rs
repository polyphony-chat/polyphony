pub mod instance {
    use crate::backend::backend::Backend;

    pub enum InstanceType {
        Fosscord,
        Discord,
    }

    pub struct Instance<B>
    where
        B: Backend,
    {
        pub name: String,
        pub url: String,
        pub instance: InstanceType,
        pub conn: B,
    }

    impl<B: Backend> Instance<B> {
        pub fn new(name: String, url: String, instance: InstanceType, conn: B) -> Instance<B> {
            Self {
                name,
                url,
                instance,
                conn,
            }
        }
    }
}
