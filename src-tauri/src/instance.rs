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
        name: String,
        url: String,
        instance: InstanceType,
        backend: B,
    }

    impl<B: Backend> Instance<B> {
        fn new(name: String, url: String, instance: InstanceType, backend: B) -> Instance<B> {
            Self {
                name,
                url,
                instance,
                backend,
            }
        }
    }
}
