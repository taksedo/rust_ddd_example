use std::env;
use std::sync::OnceLock;

use dotenvy::dotenv;
use lazy_static::lazy_static;

use crate::main::event::rabbit_message_publisher::RabbitMessagePublisher;

lazy_static! {
    pub static ref RABBITMQ_MESSAGE_PUBLISHER: RabbitMessagePublisher =
        RabbitMessagePublisher::new();
    pub static ref RABBITMQ_ADDRESS: OnceLock<String> = {
        dotenv().ok();
        let static_env: OnceLock<String> = OnceLock::new();
        let rabbitmq_address = env::var("RABBITMQ_ADDRESS").unwrap();
        static_env.get_or_init(|| rabbitmq_address);
        static_env
    };
    pub static ref RABBITMQ_QUEUE_NAME: OnceLock<String> = {
        let static_env = OnceLock::new();
        static_env.get_or_init(|| "meal_queue".to_string());
        static_env
    };
}
