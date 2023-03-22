pub mod instance {
    use crate::backend::backend::Backend;
    use crate::backendfactory::backend_factory::BackendFactory;

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

    impl<B> Instance<B> {
        fn new(name: String, url: String, instance: InstanceType) -> Self {
            let backend = BackendFactory::create_backend(instance);
            Self {
                name,
                url,
                instance,
                backend,
            }
        }
    }
}
