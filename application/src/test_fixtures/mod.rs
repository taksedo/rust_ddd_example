use lapin::{Connection, ConnectionProperties};
use std::sync::atomic::AtomicU32;
use std::sync::OnceLock;
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::images::generic::GenericImage;
use testcontainers::Container;
use tracing::debug;

#[derive(Debug)]
pub struct TestRabbitMq {
    #[allow(dead_code)]
    container: Container<'static, GenericImage>,
}
impl TestRabbitMq {
    pub async fn new() -> Self {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug");
        }

        tracing_subscriber::fmt::init();

        static DOCKER_CLIENT: OnceLock<Cli> = OnceLock::new();
        DOCKER_CLIENT.get_or_init(Cli::default);

        let msg = WaitFor::message_on_stdout("  * rabbitmq_management_agent");

        let rabbitmq_container = GenericImage::new("pivotalrabbitmq/rabbitmq-stream", "latest")
            .with_env_var(
                "RABBITMQ_SERVER_ADDITIONAL_ERL_ARGS",
                "-rabbitmq_stream advertised_host localhost",
            )
            .with_env_var("RABBITMQ_DEFAULT_USER", "guest")
            .with_env_var("RABBITMQ_DEFAULT_PASS", "guest")
            .with_wait_for(msg);
        let node: Container<'static, GenericImage> =
            DOCKER_CLIENT.get().unwrap().run(rabbitmq_container);
        let port = &node.get_host_port_ipv4(5672);
        RABBITMQ_QUEUE_NAME.get_or_init(|| {
            format!(
                "test_queue_{}_{}",
                std::process::id(),
                TEST_RABBITMQ_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            )
        });
        let test_container_rabbitmq_url = format!("amqp://guest:guest@localhost:{port}");

        RABBITMQ_ADDRESS.get_or_init(|| test_container_rabbitmq_url.clone());
        debug!(?RABBITMQ_ADDRESS);

        Self { container: node }
    }

    pub async fn conn(&self) -> lapin::Result<Connection> {
        Connection::connect(
            RABBITMQ_ADDRESS.get().unwrap(),
            ConnectionProperties::default(),
        )
        .await
    }
}

static TEST_RABBITMQ_COUNTER: AtomicU32 = AtomicU32::new(0);

pub static RABBITMQ_ADDRESS: OnceLock<String> = OnceLock::new();
pub static RABBITMQ_QUEUE_NAME: OnceLock<String> = OnceLock::new();
