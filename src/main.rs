use stocks_rust::infra::api::client::AlpacaApiClient;
use dotenv::dotenv;
use serde_json::to_string;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let client = AlpacaApiClient::new();
    let assets = client.get_all_assets().await;


    let assets_json = to_string(&assets).unwrap();
    std::fs::write("assets.json", assets_json).unwrap();
}