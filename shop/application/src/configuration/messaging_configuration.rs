use std::{env, sync::LazyLock};

use dotenvy::dotenv;

use crate::event::rabbit_message_publisher::RabbitMessagePublisher;

pub(super) static RABBITMQ_MESSAGE_PUBLISHER: LazyLock<RabbitMessagePublisher> =
    LazyLock::new(RabbitMessagePublisher::new);
pub(crate) static RABBITMQ_ADDRESS: LazyLock<String> = LazyLock::new(|| {
    dotenv().ok();
    env::var("RABBITMQ_ADDRESS").unwrap()
});
pub(crate) static RABBITMQ_QUEUE_NAME: LazyLock<String> =
    LazyLock::new(|| "meal_queue".to_string());
