use crate::infra::alpaca::asset::AlpacaCliAsset;
use crate::infra::psql::client::PsqlClient;
pub struct AssetRepository{
    alpaca_cli: AlpacaCliAsset,
    psql_cli: PsqlClient
}

impl AssetRepository {
    fn fetch_assets(&self) {}

    fn store_assets(&self) {}
}