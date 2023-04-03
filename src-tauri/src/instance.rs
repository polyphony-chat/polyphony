pub mod instance {
    use crate::backend::{Backend, URLBundle};

    pub enum InstanceType {
        Spacebar,
        Discord,
    }

    pub struct Instance<B>
    where
        B: Backend,
    {
        pub name: String,
        pub urls: URLBundle,
        pub backend_impl: B,
    }

    impl<B: Backend> Instance<B> {
        pub fn new(name: String, urls: URLBundle, backend_impl: B) -> Instance<B> {
            let urls = URLBundle::new(
                urls.get_api().to_string(),
                urls.get_wss().to_string(),
                urls.get_cdn().to_string(),
            );
            Self {
                name,
                urls,
                backend_impl,
            }
        }
    }
}
