use async_trait::async_trait;
use derive_new::new;
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Connection, ConnectionProperties};
use serde::Serialize;
use tracing::info;

#[cfg(not(test))]
use crate::main::configuration::messaging_configuration::{RABBITMQ_ADDRESS, RABBITMQ_QUEUE_NAME};
use crate::main::event::integration_message_publisher::IntegrationMessagePublisher;
#[cfg(test)]
use crate::test_fixtures::{RABBITMQ_ADDRESS, RABBITMQ_QUEUE_NAME};

#[derive(new, Debug)]
pub struct RabbitMessagePublisher;

#[async_trait]
impl IntegrationMessagePublisher for RabbitMessagePublisher {
    async fn send(
        &self,
        message: impl Serialize + Send + Sync,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let addr = RABBITMQ_ADDRESS.get().unwrap().as_str();
        let queue_name = RABBITMQ_QUEUE_NAME.get().unwrap().as_str();
        let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
        info!("Publisher connected");

        let channel = conn.create_channel().await?;
        info!(state=?conn.status().state());

        let queue = channel
            .queue_declare(
                queue_name,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;
        info!(state=?conn.status().state());
        info!(?queue, "Declared queue");

        let sent_message = serde_json::to_string(&message)?;

        let payload = sent_message.as_bytes();
        channel
            .basic_publish(
                "",
                queue_name,
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?;

        info!(?sent_message, " [x] Sent message");

        conn.close(0, "").await?;

        Ok(())
    }
}
