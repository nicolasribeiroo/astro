use actix_web::{http::KeepAlive, web, App, HttpServer};
use connectivity::postgres::PostgresManager;
use std::time::Duration;
use tokio::sync::Mutex;
use tracing_actix_web::TracingLogger;
use util::types::AsyncVoidResult;

pub mod connectivity;
pub mod routes;
pub mod util;

pub struct ServerState {
    pub postgres: PostgresManager,
}

#[tokio::main]
async fn main() -> AsyncVoidResult {
    let postgres = connectivity::postgres::PostgresManager::new();

    postgres.migrate_database().await?;

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
