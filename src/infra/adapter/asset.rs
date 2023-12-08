use crate::domain::asset::entity::Asset;

pub fn json_to_assets(json: &str) -> Result<Vec<Asset>, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_to_assets() {
        let json = r#"
        [
            {
                "id": "5cf4cbe2-95dd-47bb-9126-c96d37b2bf96",
                "class": "us_equity",
                "exchange": "NASDAQ",
                "symbol": "UBNK",
                "status": "inactive",
                "tradable": true,
                "marginable": true,
                "shortable": true,
                "easy_to_borrow": false,
                "fractionable": true
            },
            {
                "id": "83bba1f5-e216-4c8e-a56c-e9227fd13f35",
                "class": "us_equity",
                "exchange": "OTC",
                "symbol": "OSTO",
                "status": "inactive",
                "tradable": false,
                "marginable": false,
                "shortable": false,
                "easy_to_borrow": false,
                "fractionable": false
            }
        ]
        "#;
        let result = json_to_assets(json);
        assert!(result.is_ok());
        let assets = result.unwrap();
        assert_eq!(assets.len(), 2);
        let asset = &assets[0];
        assert_eq!(asset.id.to_string(), "5cf4cbe2-95dd-47bb-9126-c96d37b2bf96");
        assert_eq!(asset.version, "NO_VERSION");
        assert_eq!(asset.class, "us_equity");
        assert_eq!(asset.exchange, "NASDAQ");
        assert_eq!(asset.symbol, "UBNK");
        assert_eq!(asset.status, "inactive");
        assert_eq!(asset.tradable, true);
        assert_eq!(asset.marginable, true);
        assert_eq!(asset.shortable, true);
        assert_eq!(asset.easy_to_borrow, false);
        assert_eq!(asset.fractionable, true);
    }
}
