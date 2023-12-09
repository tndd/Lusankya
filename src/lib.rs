pub mod domain {
    pub mod asset {
        pub mod entity;
        pub mod repository;
    }
}
pub mod infra {
    pub mod adapter {
        pub mod asset;
    }
    pub mod alpaca {
        pub mod factory;
        pub mod asset;
    }
    pub mod psql {
        pub mod client;
    }
}