use std::{
    clone::Clone,
    sync::{atomic::AtomicU32, LazyLock, OnceLock},
};

use lapin::{Connection, ConnectionProperties};
use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt};
use testcontainers_modules::kafka::Kafka;
use tracing::debug;

#[derive(Debug)]
pub struct TestRabbitMq {
    #[allow(dead_code)]
    container: ContainerAsync<GenericImage>,
}

impl TestRabbitMq {
    pub async fn new() -> Self {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug");
        }

        let _ = tracing_subscriber::fmt::try_init();

        let msg = WaitFor::message_on_stdout("  * rabbitmq_management_agent");

        let rabbitmq_container = GenericImage::new("pivotalrabbitmq/rabbitmq-stream", "latest")
            .with_wait_for(msg)
            .with_env_var(
                "RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS",
                "-rabbitmq_stream advertised_host localhost",
            )
            .with_env_var("RABBITMQ_DEFAULT_USER", "guest")
            .with_env_var("RABBITMQ_DEFAULT_PASS", "guest");
        let node = rabbitmq_container.start().await.unwrap();
        let port = &node.get_host_port_ipv4(5672).await.unwrap();
        let test_container_rabbitmq_url = format!("amqp://guest:guest@localhost:{port}");

        RABBITMQ_ADDRESS_LOCK.get_or_init(|| test_container_rabbitmq_url.clone());
        debug!(?RABBITMQ_ADDRESS);

        Self { container: node }
    }

    pub async fn conn(&self) -> lapin::Result<Connection> {
        Connection::connect(&RABBITMQ_ADDRESS, ConnectionProperties::default()).await
    }
}

static TEST_RABBITMQ_COUNTER: AtomicU32 = AtomicU32::new(0);

static RABBITMQ_ADDRESS_LOCK: OnceLock<String> = OnceLock::new();
pub(crate) static RABBITMQ_ADDRESS: LazyLock<String> =
    LazyLock::new(|| RABBITMQ_ADDRESS_LOCK.get().unwrap().clone());
pub(crate) static RABBITMQ_QUEUE_NAME: LazyLock<String> = LazyLock::new(|| {
    format!(
        "test_queue_{}_{}",
        std::process::id(),
        TEST_RABBITMQ_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    )
});

pub struct TestKafka {
    #[allow(dead_code)]
    pub container: ContainerAsync<Kafka>,
}

impl TestKafka {
    pub async fn new() -> Self {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug");
        }

        let _ = tracing_subscriber::fmt::try_init();

        let node = Kafka::default().start().await.unwrap();

        let port = &node.get_host_port_ipv4(9093).await.unwrap();
        let test_container_kafka_url = format!("127.0.0.1:{port}");

        KAFKA_ADDRESS.get_or_init(|| test_container_kafka_url.clone());
        debug!(?KAFKA_ADDRESS);

        Self { container: node }
    }
}
pub static KAFKA_ADDRESS: OnceLock<String> = OnceLock::new();
