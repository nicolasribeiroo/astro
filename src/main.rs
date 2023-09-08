use actix_web::{http::KeepAlive, web, App, HttpServer};
use connectivity::postgres::PostgresManager;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt, EnvFilter, Registry};
use util::types::AsyncVoidResult;

pub mod connectivity;
pub mod routes;
pub mod structs;
pub mod util;

pub struct ServerState {
    pub postgres: PostgresManager,
}

#[tokio::main]
async fn main() -> AsyncVoidResult {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("info"));
    let fmt_layer = fmt::layer().with_target(false);
    let subscriber = Registry::default().with(env_filter).with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let postgres = connectivity::postgres::PostgresManager::new();

    postgres.migrate_database().await?;
    // postgres.drop_users_table().await?;

    tracing::info!("Starting HTTP Server on {}:{}", "0.0.0.0", "4001");

    let data = web::Data::new(Mutex::new(ServerState { postgres }));

    let data_http = web::Data::clone(&data);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&data_http))
            .wrap(TracingLogger::default())
            .configure(routes::routes)
    })
    .keep_alive(KeepAlive::Timeout(Duration::from_secs(200)))
    .bind("0.0.0.0:4001")?
    .run()
    .await
    .unwrap();

    Ok(())
}
