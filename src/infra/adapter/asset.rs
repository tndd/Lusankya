use crate::domain::asset::entity::Asset;

pub fn json_to_asset(json: &str) -> Result<Asset, serde_json::Error> {
    serde_json::from_str(json)
}