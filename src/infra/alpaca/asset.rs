use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class, Asset};
use std::sync::{Arc, RwLock};
use futures::future::join_all;
use serde_json::to_string;

pub struct AlpacaCliAsset {
    client: Arc<RwLock<Client>>
}

impl AlpacaCliAsset {
    pub fn new(client: Arc<RwLock<Client>>) -> Self {
        Self {
            client,
        }
    }

    pub async fn get_all_assets(&self) -> String {
        let assets_reqs = vec![
            self.get_request_us_equity_active(),
            self.get_request_us_equity_inactive(),
            self.get_request_crypto_active(),
            self.get_request_crypto_inactive(),
        ];
        self.get_assets(assets_reqs).await
    }

    async fn get_assets(&self, assets_reqs: Vec<AssetsReq>) -> String {
        let futures = assets_reqs.into_iter().map(|assets_req| self.get_asset(assets_req));
        let assets: Vec<Vec<Asset>> = join_all(futures).await;
        to_string(&assets.concat()).unwrap()
    }

    async fn get_asset(&self, assets_req: AssetsReq) -> Vec<Asset> {
        let client = self.client.clone();
        let client = client.read().unwrap();
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

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use crate::infra::alpaca::factory::new_apca_client;


    fn get_asset_cli() -> AlpacaCliAsset {
        dotenv().ok();
        let client = new_apca_client();
        AlpacaCliAsset::new(client)
    }

    #[test]
    fn get_request_us_equity_active_test() {
        let asset_cli = get_asset_cli();
        let request = asset_cli.get_request_us_equity_active();
        assert_eq!(request.status, Status::Active);
        assert_eq!(request.class, Class::UsEquity);
    }

    #[test]
    fn get_request_us_equity_inactive_test() {
        let asset_cli = get_asset_cli();
        let request = asset_cli.get_request_us_equity_inactive();
        assert_eq!(request.status, Status::Inactive);
        assert_eq!(request.class, Class::UsEquity);
    }

    #[test]
    fn get_request_crypto_active_test() {
        let asset_cli = get_asset_cli();
        let request = asset_cli.get_request_crypto_active();
        assert_eq!(request.status, Status::Active);
        assert_eq!(request.class, Class::Crypto);
    }

    #[test]
    fn get_request_crypto_inactive_test() {
        let asset_cli = get_asset_cli();
        let request = asset_cli.get_request_crypto_inactive();
        assert_eq!(request.status, Status::Inactive);
        assert_eq!(request.class, Class::Crypto);
    }
}