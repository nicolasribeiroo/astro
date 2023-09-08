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

        cfg.host = Some("localhost".to_string());
        cfg.port = Some(5432);
        cfg.dbname = Some("astro".to_string());
        cfg.user = Some("postgres".to_string());

        let pc = PoolConfig::new(175);
        cfg.pool = pc.into();

        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

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

        let user = User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            email: row.get(3),
        };

        Ok(user)
    }
}
