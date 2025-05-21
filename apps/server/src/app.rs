use std::sync::Arc;

use axum::Router;
use axum_responses::{response, ControllerResult};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::features::user::infrastructure::routes::router as user_router;

use crate::infrastructure::{
    constants::{check_env_vars, ALLOWED_HTTP_HEADERS, ALLOWED_HTTP_METHODS},
    database::PostgresConnection,
    di::{AppModule, AppState},
    http::logger::HttpLogger,
};

pub struct Application {
    router: Router,
}

impl Application {
    pub async fn new() -> Self {
        check_env_vars();

        let db_connection = PostgresConnection::new()
            .await
            .expect("Failed to create database connection");

        db_connection
            .migrate()
            .await
            .expect("Failed to run database migrations");

        let di_module = AppModule::builder()
            .with_component_parameters::<PostgresConnection>(db_connection.into())
            .build();

        let state = AppState {
            module: Arc::new(di_module),
        };

        let http_logger = HttpLogger::new();
        let cors_layer = CorsLayer::new()
            .allow_methods(ALLOWED_HTTP_METHODS.to_owned())
            .allow_headers(ALLOWED_HTTP_HEADERS.to_owned());

        let app_router = Router::new()
            .merge(user_router(state))
            .route("/health", axum::routing::get(Self::health_check))
            .layer(cors_layer)
            .layer(http_logger.layer);

        Application { router: app_router }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("0.0.0.0:8000").await?;

        println!("Server listening on port 8000");

        axum::serve(listener, self.router.clone()).await?;

        Ok(())
    }

    pub async fn health_check() -> ControllerResult {
        let time = chrono::Utc::now();
        let status = "running";

        response!(200, {
            "status": status,
            "time": time.to_string(),
        })
    }
}
