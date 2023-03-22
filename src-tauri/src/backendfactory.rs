struct BackendFactory {}

impl BackendFactory {
    pub fn create_backend(&self, instance: backend::InstanceType) -> impl backend::Backend {
        match instance {
            InstanceType::Fosscord => backend::FosscordBackend::new(),
            InstanceType::Discord => backend::DiscordBackend::new(),
        }
    }
}
