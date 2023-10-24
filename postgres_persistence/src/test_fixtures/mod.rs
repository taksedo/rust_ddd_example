use common_events::main::domain_event_publisher::DomainEventPublisher;
use derive_new::new;
use diesel::{sql_query, Connection, PgConnection, RunQueryDsl};
use domain::main::menu::meal::Meal;
use domain::main::menu::meal_events::DomainEventEnum;
use domain::main::menu::value_objects::meal_id::{MealId, MealIdGenerator};
use domain::test_fixtures::fixtures::{
    rnd_meal_description, rnd_meal_name, rnd_price, TestMealAlreadyExists,
};
use log::warn;
use std::sync::atomic::AtomicU32;
use std::sync::{Arc, Mutex, OnceLock};
use testcontainers::clients::Cli;
use testcontainers::core::WaitFor;
use testcontainers::images::generic::GenericImage;
use testcontainers::Container;
use url::Url;

static TEST_DB_COUNTER: AtomicU32 = AtomicU32::new(0);

#[derive(Debug)]
pub struct TestDb {
    test_container_db_url: String,
    url: String,
    curr_test_db_name: String,
    delete_on_drop: bool,
    #[allow(dead_code)]
    container: Container<'static, GenericImage>,
}
impl TestDb {
    pub fn new() -> Self {
        static DOCKER: OnceLock<Cli> = OnceLock::new();
        DOCKER.get_or_init(Cli::default);
        let msg = WaitFor::message_on_stderr("database system is ready to accept connections");

        let pg_container = GenericImage::new("postgres", "13")
            .with_env_var("POSTGRES_DB", "postgres")
            .with_env_var("POSTGRES_USER", "root")
            .with_env_var("POSTGRES_PASSWORD", "123")
            .with_wait_for(msg);

        let node: Container<'static, GenericImage> = DOCKER.get().unwrap().run(pg_container);
        let port = &node.get_host_port_ipv4(5432);

        let curr_test_db_name = format!(
            "test_db_{}_{}",
            std::process::id(),
            TEST_DB_COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
        );

        let test_container_db_url = format!("postgres://root:123@localhost:{port}/postgres");
        let mut conn = PgConnection::establish(&test_container_db_url).unwrap();
        sql_query(format!("CREATE DATABASE {};", curr_test_db_name))
            .execute(&mut conn)
            .unwrap();
        let mut url = Url::parse(&test_container_db_url).unwrap();
        url.set_path(&curr_test_db_name);
        Self {
            test_container_db_url,
            url: url.to_string(),
            curr_test_db_name,
            delete_on_drop: false,
            container: node,
        }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn conn(&self) -> PgConnection {
        PgConnection::establish(self.url.as_str()).unwrap()
    }

    pub fn leak(&mut self) {
        self.delete_on_drop = false;
    }
}

impl Default for TestDb {
    fn default() -> Self {
        Self::new()
    }
}
impl Drop for TestDb {
    fn drop(&mut self) {
        if !self.delete_on_drop {
            warn!("TestDb leaking database {}", self.curr_test_db_name);
            return;
        }
        let mut conn = diesel_logger::LoggingConnection::new(
            PgConnection::establish(&self.test_container_db_url).unwrap(),
        );
        sql_query(format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}'",
            self.curr_test_db_name
        ))
        .execute(&mut conn)
        .unwrap();
        sql_query(format!("DROP DATABASE {}", self.curr_test_db_name))
            .execute(&mut conn)
            .unwrap();
    }
}

#[derive(new, Debug, Default)]
pub struct MockEventPublisher {
    events: Vec<DomainEventEnum>,
}

impl MockEventPublisher {
    pub fn verify_contains(&self, events: Vec<DomainEventEnum>) {
        let matching = &self
            .events
            .iter()
            .zip(&events)
            .filter(|&(a, b)| a == b)
            .count();
        assert_eq!(matching, &0_usize)
    }

    pub fn verify_event_is_empty(&self) {
        assert!(&self.events.is_empty())
    }
}

impl DomainEventPublisher<DomainEventEnum> for MockEventPublisher {
    fn publish(&mut self, events: &Vec<DomainEventEnum>) {
        self.events.extend(events.clone())
    }
}

#[derive(Debug, new, Default)]
pub(crate) struct TestMealIdGenerator {
    pub meal_id: MealId,
}

impl MealIdGenerator for TestMealIdGenerator {
    fn generate(&mut self) -> MealId {
        self.meal_id
    }
}

pub fn rnd_meal_with_event(meal_id: MealId) -> Meal {
    let meal_name = rnd_meal_name();
    let meal_description = rnd_meal_description();
    let meal_price = rnd_price();
    let id_generator = Arc::new(Mutex::new(TestMealIdGenerator::new(meal_id)));

    Meal::add_meal_to_menu(
        Arc::clone(&id_generator) as _,
        Arc::new(Mutex::new(TestMealAlreadyExists { value: false })) as _,
        meal_name,
        meal_description,
        meal_price,
    )
    .unwrap()
}
