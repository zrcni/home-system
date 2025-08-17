use bytes::Bytes;

use crate::{
    conditions::{MongoDBConditionsRepo, handle_living_room_conditions_update},
    mqtt::MqttTopics,
};
use log;

pub struct MqttHandler {
    conditions_repo: MongoDBConditionsRepo,
}

impl MqttHandler {
    pub fn new(conditions_repo: MongoDBConditionsRepo) -> Self {
        MqttHandler { conditions_repo }
    }

    pub async fn handle_event(&self, topic: String, payload: Bytes) {
        match topic.as_str() {
            MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED => {
                let payload_str: &str = str::from_utf8(&payload).unwrap();
                handle_living_room_conditions_update(payload_str, &self.conditions_repo).await
            }
            _ => {
                log::error!("Unhandled topic: {}", topic);
            }
        }
    }
}

pub fn create_new_mqtt_handler(conditions_repo: MongoDBConditionsRepo) -> MqttHandler {
    MqttHandler::new(conditions_repo)
}
