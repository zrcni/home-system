use actix_web::{Error, HttpResponse, web};
use serde_json::json;

use crate::{mqtt::MqttTopics, startup::AppState};

pub async fn publish_mqtt_mock(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let payload = json!({"humidity":20.1,"temperature":23.2,"device_id":"mock_device","timestamp":1665779082868i64,"client_id":"mock_client"});

    let result = state.mqtt_client.publish(
        MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED,
        rumqttc::QoS::AtLeastOnce,
        false,
        payload.to_string(),
    );

    if result.is_err() {
        eprintln!("Failed to publish MQTT message: {}", result.unwrap_err());
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        Ok(HttpResponse::Ok().json(payload))
    }
}
