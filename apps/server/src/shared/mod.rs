pub mod infrastructure {
    pub mod cache;
    pub mod database;
    pub mod di;
    pub mod http {
        pub mod extractors;
        pub mod logger;

        pub fn gen_etag() -> String {
            uuid::Uuid::new_v4().to_string()
        }
    }
}

pub mod constants;

pub mod domain {
    pub mod cache;
}
