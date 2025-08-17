use actix_web::{Error, HttpResponse, web};
use serde_json::json;
use log;
use crate::{mqtt::MqttTopics, startup::AppState};

pub async fn publish_mqtt_mock(state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let payload = json!({"humidity":20.1,"temperature":23.2,"device_id":"mock_device","timestamp":1665779082868i64,"client_id":"mock_client"});

    let result = state
        .mqtt_client
        .publish(
            MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED,
            rumqttc::QoS::AtLeastOnce,
            false,
            payload.to_string(),
        )
        .await;

    if result.is_err() {
        log::error!("Failed to publish MQTT message: {}", result.unwrap_err());
        Ok(HttpResponse::InternalServerError().finish())
    } else {
        log::info!("Published MQTT message: {}", payload);
        Ok(HttpResponse::Ok().json(payload))
    }
}
