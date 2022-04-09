use std::time::Duration;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn build_db_pool() -> DatabaseConnection {
    let db_url = std::env::var("DB.URL").expect("No database URL was found.");

    let mut opt = ConnectOptions::new(db_url.to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    Database::connect(opt)
        .await
        .expect("Could not connect to database.")
}
