use apca::ApiInfo;
use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class, Asset};
use std::sync::Arc;
use futures::Future;
use tokio::try_join

pub struct AlpacaApiClient {
    client: Arc<Client>
}

impl AlpacaApiClient {
    pub fn new() -> Self {
        let api_info = ApiInfo::from_env().unwrap();
        let client = Arc::new(Client::new(api_info));
        Self { client }
    }

    async fn get_assets_part(&self, assets_req: AssetsReq) -> Vec<Asset> {}

    async fn get_assets(&self, assets_reqs: Vec<AssetsReq>) -> Vec<Asset> {}

    // async fn get_assets(&self, assets_req: AssetsReq) -> Vec<Asset> {
    //     let client = Arc::clone(&self.client);
    //     let assets = client
    //         .issue::<apca::api::v2::assets::Get>(&assets_req)
    //         .await
    //         .unwrap();
    //     assets
    // }

    // fn _get_assets(&self, assets_req: AssetsReq) -> impl Future<Output = Result<Vec<Asset>, ()>> {
    //     let client = Arc::clone(&self.client);
    //     async move {
    //         match client.issue::<apca::api::v2::assets::Get>(&assets_req).await {
    //             Ok(assets) => Ok(assets),
    //             Err(_) => Ok(Vec::new()),  // エラーが発生した場合は空のベクタを返す
    //         }
    //     }
    // }

    fn get_req_us_equity_active(&self) -> AssetsReq {
        let assets_req = AssetsReq {
            status: Status::Active,
            class: Class::UsEquity,
        };
        assets_req
    }

    fn get_req_us_equity_inactive(&self) -> AssetsReq {
        let assets_req = AssetsReq {
            status: Status::Inactive,
            class: Class::UsEquity,
        };
        assets_req
    }

    fn get_req_crypto_active(&self) -> AssetsReq {
        let assets_req = AssetsReq {
            status: Status::Active,
            class: Class::Crypto,
        };
        assets_req
    }

    fn get_req_crypto_inactive(&self) -> AssetsReq {
        let assets_req = AssetsReq {
            status: Status::Inactive,
            class: Class::Crypto,
        };
        assets_req
    }
}
