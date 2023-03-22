pub mod backend_factory {
    use crate::backend::backend;
    use crate::instance::instance::InstanceType;
    pub struct BackendFactory {}

    /// The BackendFactory handles additional logic for backend instance creation.
    /// For example, it creates a new backend instance based on the instance type.
    impl BackendFactory {
        pub fn create_backend(instance_type: InstanceType) -> impl backend::Backend {
            let backend: backend::Backend;
            match instance_type {
                InstanceType::Fosscord => backend = backend::FosscordBackend::new(),
                InstanceType::Discord => backend = backend::DiscordBackend::new(),
            }
            backend
        }
    }
}
