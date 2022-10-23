use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize, Serialize)]
pub struct WalletHistory {
    pub token: String,
    pub block: i64,
    pub address: String,
    pub balance: u64,
}

impl From<Value> for WalletHistory {
    fn from(value: Value) -> WalletHistory {
        let profile = serde_json::from_value::<WalletHistory>(value);
        if let Ok(profile) = profile {
            return profile;
        }

        panic!("Failed to parse value to WalletHistory")
    }
}

impl Into<Value> for WalletHistory {
    fn into(self) -> Value {
        serde_json::from_str::<Value>(&serde_json::to_string(&self).unwrap()).unwrap()
    }
}
