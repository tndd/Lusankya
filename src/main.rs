use stocks_rust::infra::api::client::AlpacaApiClient;
use dotenv::dotenv;
use serde_json::to_string;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let alpaca_cli = AlpacaApiClient::new();
    let assets = alpaca_cli.get_all_assets().await;


    let assets_json = to_string(&assets).unwrap();
    std::fs::write("assets.json", assets_json).unwrap();
}