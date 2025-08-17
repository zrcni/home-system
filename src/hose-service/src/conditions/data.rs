use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConditionData {
    // saved as string, but actually float "XX.X"
    pub temperature: String,
    // saved as string, but actually float "XX.X"
    pub humidity: String,
    pub device_id: String,
    pub timestamp: i64,
    pub client_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LivingRoomConditionUpdated {
    pub temperature: f32,
    pub humidity: f32,
    pub device_id: String,
    pub timestamp: i64,
    pub client_id: String,
}

impl From<LivingRoomConditionUpdated> for ConditionData {
    fn from(update: LivingRoomConditionUpdated) -> Self {
        ConditionData {
            temperature: format!("{:.1}", update.temperature),
            humidity: format!("{:.1}", update.humidity),
            device_id: update.device_id,
            timestamp: update.timestamp,
            client_id: update.client_id,
        }
    }
}
