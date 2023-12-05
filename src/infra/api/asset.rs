use apca::ApiInfo;
use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class};

pub struct AlpacaApiClientAsset {
    client: Client
}

impl AlpacaApiClientAsset {
    pub fn get_assets() {
        let assets_req = AssetsReq {
            status: Status::Active,
            class: Class::UsEquity,
        };
        let assets = client
            .issue::<apca::api::v2::assets::Get>(&assets_req)
            .await
            .unwrap();
        assets
    }
}