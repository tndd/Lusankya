use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone};

#[derive(Deserialize, Serialize)]
pub struct Asset {
    pub id: Uuid,
    #[serde(default = "default_version")]
    pub version: DateTime<Utc>,
    pub class: String,
    pub exchange: String,
    pub symbol: String,
    pub status: String,
    pub tradable: bool,
    pub marginable: bool,
    pub shortable: bool,
    pub easy_to_borrow: bool,
    pub fractionable: bool,
}

fn default_version() -> DateTime<Utc> {
    Utc.timestamp_opt(0, 0).unwrap()
}