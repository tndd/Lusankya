use crate::infra::alpaca::asset::AlpacaCliAsset;
pub struct AssetRepository{
    alpaca_cli: AlpacaCliAsset,
}

impl AssetRepository {
    fn fetch_assets(&self) {}

    fn store_assets(&self) {}

    fn retrive_assets(&self) {}
}
