pub mod application {
    pub mod interfaces {
        pub mod create;
        pub mod delete;
        pub mod get;
        pub mod update;
    }
    pub mod usecases {
        pub mod create;
        pub mod delete;
        pub mod get;
        pub mod update;
    }
    pub mod services {
        pub mod password;
    }
}

pub mod domain {
    pub mod entity;
    pub mod errors;
    pub mod repository;
}

pub mod infrastructure {
    pub mod controllers;
    pub mod dtos;
    pub mod errors;
    pub mod models;
    pub mod repository;
    pub mod routes;
}
