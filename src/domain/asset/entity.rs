use diesel::prelude::*;
use uuid::Uuid;
use chrono::NaiveDateTime;
use crate::schema::asset;

#[derive(Insertable, Queryable, Identifiable)]
#[table_name="asset"]
#[primary_key(id, version)]
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