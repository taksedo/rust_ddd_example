// use diesel::result::Error;
// use diesel::sql_types::Uuid;
// use diesel::{Connection, PgConnection};
// use pg_embed::pg_enums::PgAuthMethod;
// use pg_embed::pg_errors::PgEmbedError;
// use pg_embed::pg_fetch::{PgFetchSettings, PG_V13};
// use pg_embed::pg_types::PgResult;
// use pg_embed::postgres::{PgEmbed, PgSettings};
// use serial_test::serial;
// use std::path::PathBuf;
// use std::time::Duration;
//
// const DATABASE_NAME: &str = "foo";
//
// async fn setup_db(
//     port: u16,
//     database_dir: PathBuf,
//     persistent: bool,
//     migration_dir: Option<PathBuf>,
// ) -> Result<PgEmbed, PgEmbedError> {
//     let pg_settings = PgSettings {
//         database_dir,
//         port,
//         user: "postgres".to_string(),
//         password: "password".to_string(),
//         auth_method: PgAuthMethod::MD5,
//         persistent,
//         timeout: Some(Duration::from_secs(10)),
//         migration_dir,
//     };
//     let fetch_settings = PgFetchSettings {
//         version: PG_V13,
//         ..Default::default()
//     };
//     let mut pg = PgEmbed::new(pg_settings, fetch_settings).await?;
//     pg.setup().await?;
//
//     Ok(pg)
// }
//
// async fn setup() -> Result<PgEmbed, PgEmbedError> {
//     let mut pg = setup_db(
//         5434,
//         PathBuf::from("data_test/db"),
//         false,
//         Some(PathBuf::from("migration_test")),
//     )
//     .await?;
//
//     pg.start_db().await?;
//     pg.create_database(&DATABASE_NAME).await?;
//     pg.migrate(&DATABASE_NAME).await?;
//
//     Ok(pg)
// }
//
// async fn run_test<T>(test: T) -> ()
// where
//     T: FnOnce(PgConnection) -> () + std::panic::UnwindSafe,
// {
//     let mut pg = setup().await.unwrap();
//     let db_uri = pg.full_db_uri(DATABASE_NAME);
//     let connection =
//         PgConnection::establish(&db_uri).expect(&format!("Error connecting to database"));
//
//     let result = std::panic::catch_unwind(|| test(connection));
//
//     pg.stop_db().await.unwrap();
//
//     // no panics (or result.is_err())
//     assert!(result.is_ok())
// }
//
// fn add_by_id(id_value: Uuid, conn: &PgConnection) -> Result<Account, Error> {
//     let mut data = Foo::default();
//     data.id = id_value;
//     data.add(&conn)
// }
//
// #[tokio::test]
// #[serial]
// async fn add() {
//     run_test(|connection| {
//         let id_value = Uuid::new_v4();
//         let result = add_by_id(id_value, &connection).unwrap();
//
//         assert_eq!(result.id, id_value);
//     })
//     .await;
// }
//
// #[tokio::test]
// #[serial]
// async fn get_by_id() {
//     run_test(|connection| {
//         let id_value = Uuid::new_v4();
//         add_by_id(id_value, &connection).unwrap();
//         let result = Foo::get_by_id(&connection, id_value).unwrap();
//         assert_eq!(result.id, id_value);
//     })
//     .await;
// }
mod postgres_meal_id_generator_test;
