use async_trait::async_trait;
use serde::Serialize;

#[async_trait]
pub(crate) trait IntegrationMessagePublisher {
    async fn send(
        &self,
        message: impl Serialize + Send + Sync,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
