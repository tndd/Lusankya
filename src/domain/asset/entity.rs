use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::asset;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Queryable, Identifiable)]
#[diesel(table_name = asset)]
#[diesel(primary_key(id, version))]
pub struct Asset {
    id: Uuid,
    version: NaiveDateTime,
    class: String,
    exchange: String,
    symbol: String,
    status: String,
    tradable: bool,
    marginable: bool,
    shortable: bool,
    easy_to_borrow: bool,
    fractionable: bool,
}