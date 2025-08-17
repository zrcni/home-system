use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConditionData {
    // saved as string, but actually float "XX.X"
    temperature: String,
    // saved as string, but actually float "XX.X"
    humidity: String,
    device_id: String,
    timestamp: i64,
    client_id: String,
}

#[derive(Deserialize)]
pub struct LivingRoomConditionUpdated {
    temperature: f32,
    humidity: f32,
    device_id: String,
    timestamp: i64,
    client_id: String,
}

impl From<LivingRoomConditionUpdated> for ConditionData {
    fn from(update: LivingRoomConditionUpdated) -> Self {
        ConditionData {
            temperature: update.temperature.to_string(),
            humidity: update.humidity.to_string(),
            device_id: update.device_id,
            timestamp: update.timestamp,
            client_id: update.client_id,
        }
    }
}
