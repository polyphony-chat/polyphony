pub mod instance {
    use crate::backend::backend::Backend;
    use url::{ParseError, Url};

    pub enum InstanceType {
        Spacebar,
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
            let url = match Url::parse(&url) {
                Ok(url) => url,
                Err(ParseError::RelativeUrlWithoutBase) => {
                    let url = format!("http://{}", url);
                    Url::parse(&url).unwrap()
                }
                Err(_) => panic!("Invalid URL"),
            };
            let url = url.to_string();
            Self {
                name,
                url,
                instance,
                conn,
            }
        }
    }
}
