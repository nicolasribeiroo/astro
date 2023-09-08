use crate::util::types::{AsyncResult, AsyncVoidResult};
use deadpool_postgres::{Config, Pool, PoolConfig, Runtime};
use serde::{Deserialize, Serialize};
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct PostgresManager {
    pub pool: Pool,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub email: String,
}

impl PostgresManager {
    pub fn new() -> PostgresManager {
        let mut cfg = Config::new();

        let host = std::env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = std::env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let dbname = std::env::var("POSTGRES_DBNAME").unwrap_or_else(|_| "astro".to_string());
        let password =
            std::env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let user = std::env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".to_string());

        cfg.host = Some(host);
        cfg.port = Some(port.parse::<u16>().unwrap());
        cfg.dbname = Some(dbname);
        cfg.password = Some(password);
        cfg.user = Some(user);

        let pc = PoolConfig::new(175);
        cfg.pool = pc.into();

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

        tracing::info!("Postgres connection pool created");

        Self { pool }
    }

    pub async fn migrate_database(&self) -> AsyncVoidResult {
        let client = self.pool.get().await?;

        let _ = client
            .execute(
                "CREATE TABLE IF NOT EXISTS users (
                id VARCHAR(255) PRIMARY KEY,
                username VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                email VARCHAR(255) NOT NULL
            )",
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn drop_users_table(&self) -> AsyncVoidResult {
        let client = self.pool.get().await?;

        let _ = client.execute("DROP TABLE users", &[]).await?;

        Ok(())
    }

    pub async fn insert_new_user(&self, user: User) -> AsyncResult<User> {
        let client = self.pool.get().await?;

        let _ = client
            .execute(
                "INSERT INTO users (id, username, password, email) VALUES ($1, $2, $3, $4) RETURNING *",
                &[&user.id, &user.username, &user.password, &user.email],
            )
            .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: String) -> AsyncResult<User> {
        let client = self.pool.get().await?;

        let row = client
            .query_one("SELECT * FROM users WHERE id = $1", &[&id])
            .await?;

        if row.is_empty() {
            return Err("User not found".into());
        }

        let user = User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            email: row.get(3),
        };

        Ok(user)
    }

    pub async fn delete_user_by_id(&self, id: String) -> AsyncVoidResult {
        let client = self.pool.get().await?;

        let _ = client
            .execute("DELETE FROM users WHERE id = $1", &[&id])
            .await?;

        Ok(())
    }
}
