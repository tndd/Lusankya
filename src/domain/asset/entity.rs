use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize, Serialize)]
pub struct Asset {
    pub id: Uuid,
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
