pub mod domain {
    pub mod asset {
        pub mod entity;
        pub mod repository;
    }
}
pub mod infra {
    pub mod alpaca {
        pub mod asset;
    }
    pub mod psql {
        pub mod client;
    }
}
pub mod schema;