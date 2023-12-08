use diesel::prelude::*;
use uuid::Uuid;
use crate::schema::asset;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Queryable, Identifiable)]
#[diesel(table_name = asset)]
#[diesel(primary_key(id, version))]
pub struct Asset {
    pub id: Uuid,
    #[serde(default = "default_version")]
    pub version: String,
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

fn default_version() -> String {
    "NO_VERSION".to_string()
}