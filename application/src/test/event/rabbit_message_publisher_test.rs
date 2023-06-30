#![allow(unused_imports)]
#![allow(dead_code)]
use async_trait::async_trait;
use derive_new::new;
use serde::{Deserialize, Serialize};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::main::event::{
    integration_message_publisher::IntegrationMessagePublisher,
    rabbit_message_publisher::RabbitMessagePublisher,
};
use futures_lite::stream::StreamExt;
use lapin::message::{BasicReturnMessage, Delivery, DeliveryResult};
use lapin::protocol::{AMQPErrorKind, AMQPSoftError};
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, Result,
};

#[tokio::test]
async fn message_sent_successfully() -> std::result::Result<(), Box<dyn std::error::Error>> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    tracing_subscriber::fmt::init();
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
    pub async fn receive(&self) -> std::result::Result<SimpleDto, Box<dyn std::error::Error>> {
        let addr = "amqp://127.0.0.1:5672";
        let queue_name = "test_rabbitmq_lapin_example_queue";
        let conn = Connection::connect(addr, ConnectionProperties::default()).await?;
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
        Ok(message)
    }
}
