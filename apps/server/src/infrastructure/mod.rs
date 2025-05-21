pub mod database;
pub mod di;

pub mod http {
    pub mod extractors;
    pub mod logger;
}

pub mod constants {
    use axum::http::{HeaderName, Method};
    use lazy_static::lazy_static;
    use std::env;

    fn get_env_var(key: &str) -> String {
        env::var(key)
            .unwrap_or_else(|_| panic!("Environment variable {} not set", key))
    }

    lazy_static! {
        pub static ref DATABASE_URL: String = get_env_var("POSTGRES_DATABASE_URL");
        pub static ref JWT_SECRET: String = get_env_var("JWT_SECRET");
        pub static ref ALLOWED_HTTP_HEADERS: Vec<HeaderName> = vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ];
        pub static ref ALLOWED_HTTP_METHODS: Vec<Method> = vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH
        ];
    }

    pub fn check_env_vars() {
        let _ = DATABASE_URL.clone();
        let _ = JWT_SECRET.clone();
    }
}
