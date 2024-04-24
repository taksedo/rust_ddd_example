use std::error::Error;

use async_trait::async_trait;
use derive_new::new;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};
use serde::Serialize;
use tracing::info;

#[cfg(not(test))]
use crate::configuration::messaging_configuration::{RABBITMQ_ADDRESS, RABBITMQ_QUEUE_NAME};
use crate::event::integration_message_publisher::IntegrationMessagePublisher;
#[cfg(test)]
use crate::test_fixtures::{RABBITMQ_ADDRESS, RABBITMQ_QUEUE_NAME};

#[derive(new, Debug)]
pub(crate) struct RabbitMessagePublisher;

#[async_trait]
impl IntegrationMessagePublisher for RabbitMessagePublisher {
    async fn send(&self, message: impl Serialize + Send + Sync) -> Result<(), Box<dyn Error>> {
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

#[cfg(test)]
mod test {
    use std::str::from_utf8;

    use futures_lite::StreamExt;
    use lapin::options::{BasicAckOptions, BasicConsumeOptions};
    use serde::Deserialize;

    use super::*;
    use crate::test_fixtures::TestRabbitMq;

    #[tokio::test]
    async fn message_sent_successfully() -> Result<(), Box<dyn Error>> {
        let rmq = TestRabbitMq::new().await;
        let _ = rmq.conn().await?;

        let sent_message = SimpleDto::new();

        let publisher = RabbitMessagePublisher::new();
        publisher.send(sent_message.clone()).await?;

        let receiver = MockReceiver::new();
        let received_message = receiver.receive().await?;

        assert_eq!(sent_message, received_message);

        Ok(())
    }

    #[derive(new, Serialize, Deserialize, Debug, PartialEq, Clone)]
    struct SimpleDto {
        #[new(value = "String::from(\"bar\")")]
        foo: String,
    }

    #[derive(new)]
    struct MockReceiver {}

    impl MockReceiver {
        async fn receive(&self) -> Result<SimpleDto, Box<dyn Error>> {
            let queue_name = RABBITMQ_QUEUE_NAME.get().unwrap().as_str();
            let conn = Connection::connect(
                RABBITMQ_ADDRESS.get().unwrap(),
                ConnectionProperties::default(),
            )
            .await?;
            let channel = conn.create_channel().await?;
            info!("Consumer connected");

            let queue = channel
                .queue_declare(
                    queue_name,
                    QueueDeclareOptions::default(),
                    FieldTable::default(),
                )
                .await?;
            info!(state=?conn.status().state());
            info!(?queue, "Declared queue for consumer");

            let mut consumer = channel
                .basic_consume(
                    queue_name,
                    "consumer",
                    BasicConsumeOptions::default(),
                    FieldTable::default(),
                )
                .await
                .expect("basic_consume");

            let delivery = consumer.next().await.unwrap();
            let mut message = SimpleDto::new();
            if let Ok(delivery) = delivery {
                message = serde_json::from_str(from_utf8(&delivery.data)?)?;
                info!(" [x] Received {:?}", &message);
                delivery
                    .ack(BasicAckOptions::default())
                    .await
                    .expect("basic_ack");
            }
            conn.close(0, "").await?;
            Ok(message)
        }
    }
}
