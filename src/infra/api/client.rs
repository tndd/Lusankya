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
        let useq_active_ft = self.get_assets_us_equity_active();
        let crypto_active_ft = self.get_assets_crypto_active();
        let useq_inactive_ft = self.get_assets_us_equity_inactive();
        let crypto_inactive_ft = self.get_assets_crypto_inactive();
        let (
            mut us_equity_active_assets,
            mut crypto_active_assets,
            mut us_equity_inactive_assets,
            mut crypto_inactive_assets
        ) = join!(
            useq_active_ft,
            crypto_active_ft,
            useq_inactive_ft,
            crypto_inactive_ft
        );
        let mut all_assets = Vec::new();
        all_assets.append(&mut us_equity_active_assets);
        all_assets.append(&mut crypto_active_assets);
        all_assets.append(&mut us_equity_inactive_assets);
        all_assets.append(&mut crypto_inactive_assets);
        all_assets
    }

    pub async fn get_active_assets(&self) -> Vec<Asset> {
        let useq_active_ft = self.get_assets_us_equity_active();
        let crypto_active_ft = self.get_assets_crypto_active();
        let (
            mut us_equity_active_assets,
            mut crypto_active_assets,
        ) = join!(
            useq_active_ft,
            crypto_active_ft,
        );
        let mut all_assets = Vec::new();
        all_assets.append(&mut us_equity_active_assets);
        all_assets.append(&mut crypto_active_assets);
        all_assets
    }
}
