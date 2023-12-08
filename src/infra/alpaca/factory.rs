use apca::{ApiInfo, Client};
use std::sync::{Arc, RwLock};

pub fn new_apca_client() -> Arc<RwLock<Client>> {
    // [Note] Run dotenv first.
    // Fill env variable APCA_API_KEY_ID & APCA_API_SECRET_KEY.
    let apca_api_info = ApiInfo::from_env().unwrap();
    Arc::new(RwLock::new(Client::new(apca_api_info)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_apca_client() {
        // Prepare env vars.
        let apca_key_id = "dummy_key_id";
        let apca_secret = "dummy_secret_key";
        std::env::set_var("APCA_API_KEY_ID", apca_key_id);
        std::env::set_var("APCA_API_SECRET_KEY", apca_secret);
        // Create new instance.
        let apca_client = new_apca_client();
        let apca_client = apca_client.read().unwrap();
        // Do test.
        assert_eq!(apca_key_id, apca_client.api_info().key_id.as_str());
        assert_eq!(apca_secret, apca_client.api_info().secret.as_str());
    }
}
