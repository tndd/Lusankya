use apca::ApiInfo;
use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_info = ApiInfo::from_env().unwrap();
    let client = Client::new(api_info);

    let assets_req = AssetsReq {
        status: Status::Active,
        class: Class::UsEquity,
    };
    let assets = client.issue::<apca::api::v2::assets::Get>(&assets_req).await.unwrap();

    for asset in assets {
        println!("{} is tradable: {}", asset.symbol, asset.tradable);
    }
}