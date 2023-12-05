use apca::ApiInfo;
use apca::Client;
use apca::api::v2::assets::AssetsReq;
use apca::api::v2::asset::{Status, Class, Asset};
use futures::join;

pub struct AlpacaApiClient {
    client: Client
}

impl AlpacaApiClient {
    pub fn new() -> Self {
        let api_info = ApiInfo::from_env().unwrap();
        let client = Client::new(api_info);
        Self { client }
    }

    async fn get_assets(&self, assets_req: AssetsReq) -> Vec<Asset> {
        let assets = self.client
            .issue::<apca::api::v2::assets::Get>(&assets_req)
            .await
            .unwrap();
        assets
    }

    pub async fn get_assets_us_equity_active(&self) -> Vec<Asset> {
        let assets_req = AssetsReq {
            status: Status::Active,
            class: Class::UsEquity,
        };
        self.get_assets(assets_req).await
    }

    pub async fn get_assets_us_equity_inactive(&self) -> Vec<Asset> {
        let assets_req = AssetsReq {
            status: Status::Inactive,
            class: Class::UsEquity,
        };
        self.get_assets(assets_req).await
    }

    pub async fn get_assets_crypto_active(&self) -> Vec<Asset> {
        let assets_req = AssetsReq {
            status: Status::Active,
            class: Class::Crypto,
        };
        self.get_assets(assets_req).await
    }

    pub async fn get_assets_crypto_inactive(&self) -> Vec<Asset> {
        let assets_req = AssetsReq {
            status: Status::Inactive,
            class: Class::Crypto,
        };
        self.get_assets(assets_req).await
    }

    pub async fn get_all_assets(&self) -> Vec<Asset> {
        let f1 = self.get_assets_us_equity_active();
        let f2 = self.get_assets_crypto_active();
        let (mut assets1, mut assets2) = join!(f1, f2);
        assets1.append(&mut assets2);
        assets1
    }
}
