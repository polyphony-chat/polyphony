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
        pub connection: B,
    }

    impl<B: Backend> Instance<B> {
        pub fn new(name: String, urls: URLBundle, connection: B) -> Instance<B> {
            let urls = URLBundle::new(
                urls.get_api().to_string(),
                urls.get_wss().to_string(),
                urls.get_cdn().to_string(),
            );
            Self {
                name,
                urls,
                connection,
            }
        }
    }
}
