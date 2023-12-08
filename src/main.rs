use stocks_rust::infra::alpaca::asset::AlpacaCliAsset;
use apca::{ApiInfo, Client};
use dotenv::dotenv;
use serde_json::to_string;
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let apca_api_info = ApiInfo::from_env().unwrap();
    let apca_client = Arc::new(RwLock::new(Client::new(apca_api_info)));

    let alpaca_cli_asset = AlpacaCliAsset::new(apca_client.clone());
    let assets = alpaca_cli_asset.get_all_assets().await;

    let assets_json = to_string(&assets).unwrap();
    std::fs::write("assets.json", assets_json).unwrap();
}