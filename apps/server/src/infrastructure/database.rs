use std::time::Duration;

use shaku::{Component, Interface};
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::infrastructure::constants::DATABASE_URL;

pub trait DatabaseConnection: Interface {
    fn get_pool(&self) -> &PgPool;
}

#[derive(Component)]
#[shaku(interface = DatabaseConnection)]
pub struct PostgresConnection {
    pub pool: PgPool,
}

impl PostgresConnection {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .min_connections(1)
            .max_connections(10)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&DATABASE_URL)
            .await?;

        Ok(Self { pool })
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!("./config/migrations")
            .run(&self.pool)
            .await?;

        println!("Database migrations completed successfully.");

        Ok(())
    }
}

impl DatabaseConnection for PostgresConnection {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}

impl Into<PostgresConnectionParameters> for PostgresConnection {
    fn into(self) -> PostgresConnectionParameters {
        PostgresConnectionParameters { pool: self.pool }
    }
}
