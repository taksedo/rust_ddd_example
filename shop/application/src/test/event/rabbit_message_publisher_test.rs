use derive_new::new;
use futures_lite::stream::StreamExt;
use lapin::{options::*, types::FieldTable, Connection, ConnectionProperties};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::main::event::{
    integration_message_publisher::IntegrationMessagePublisher,
    rabbit_message_publisher::RabbitMessagePublisher,
};
use crate::test_fixtures::{TestRabbitMq, RABBITMQ_ADDRESS, RABBITMQ_QUEUE_NAME};

#[tokio::test]
async fn message_sent_successfully() -> Result<(), Box<dyn std::error::Error>> {
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
    pub async fn receive(&self) -> Result<SimpleDto, Box<dyn std::error::Error>> {
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
            message = serde_json::from_str(std::str::from_utf8(&delivery.data)?)?;
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
