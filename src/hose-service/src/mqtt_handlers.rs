use crate::mqtt::MqttTopics;
use rumqttc::Publish;

pub fn handle_incoming_mqtt_event(event: Publish) {
    match event.topic.as_str() {
        MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED => {
            let payload_str: &str = str::from_utf8(&event.payload).unwrap();
            handle_living_room_conditions_update(payload_str);
        }
        _ => {
            println!("Unhandled topic: {}", event.topic);
        }
    }
}

// #[derive(serde::Deserialize)]
// pub struct LivingRoomConditionUpdated {
//     temperature: f32,
//     humidity: f32,
//     device_id: String,
//     timestamp: i64,
//     client_id: String,
// }

fn handle_living_room_conditions_update(payload: &str) {
    // let data = serde_json::from_str::<LivingRoomConditionUpdated>(payload).unwrap();
    println!("Living room conditions updated: {}", payload);
    // Further processing can be done here
}
