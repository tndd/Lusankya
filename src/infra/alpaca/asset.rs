use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class, Asset};
use std::sync::Arc;
use futures::future::join_all;

pub struct AlpacaCliAsset {
    client: Arc<Client>
}

impl AlpacaCliAsset {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
        }
    }

    pub async fn get_all_assets(&self) -> Vec<Asset> {
        let assets_reqs = vec![
            self.get_request_us_equity_active(),
            self.get_request_us_equity_inactive(),
            self.get_request_crypto_active(),
            self.get_request_crypto_inactive(),
        ];
        self.get_assets(assets_reqs).await
    }

    async fn get_assets(&self, assets_reqs: Vec<AssetsReq>) -> Vec<Asset> {
        let futures = assets_reqs.into_iter().map(|assets_req| self.get_asset(assets_req));
        let assets: Vec<Vec<Asset>> = join_all(futures).await;
        assets.concat()
    }

    async fn get_asset(&self, assets_req: AssetsReq) -> Vec<Asset> {
        let client = Arc::clone(&self.client);
        client
            .issue::<apca::api::v2::assets::Get>(&assets_req)
            .await
            .unwrap()
    }

    fn get_request_us_equity_active(&self) -> AssetsReq {
        AssetsReq {
            status: Status::Active,
            class: Class::UsEquity,
        }
    }

    fn get_request_us_equity_inactive(&self) -> AssetsReq {
        AssetsReq {
            status: Status::Inactive,
            class: Class::UsEquity,
        }
    }

    fn get_request_crypto_active(&self) -> AssetsReq {
        AssetsReq {
            status: Status::Active,
            class: Class::Crypto,
        }
    }

    fn get_request_crypto_inactive(&self) -> AssetsReq {
        AssetsReq {
            status: Status::Inactive,
            class: Class::Crypto,
        }
    }
}
