mod common;
use std::time::SystemTime;

use bytes::Bytes;
use common::get_settings;
use hose_service::{
    conditions::{LivingRoomConditionUpdated, create_mongodb_conditions_repo},
    mongodb::create_mongodb_client,
    mqtt_handlers::create_new_mqtt_handler,
};

fn unix_now() -> i64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}

#[tokio::test]
pub async fn test_handle_living_room_condition_updated() {
    let settings = get_settings();

    let conditions_repo = create_mongodb_conditions_repo(
        create_mongodb_client(settings.conditions_mongodb_uri.clone())
            .await
            .expect("Failed to create MongoDB client"),
        settings.conditions_db_name.clone(),
        settings.conditions_db_collection_name.clone(),
    )
    .unwrap();

    let mqtt_handler = create_new_mqtt_handler(conditions_repo.clone());

    let timestamp = unix_now();

    let event = LivingRoomConditionUpdated {
        temperature: 22.5,
        humidity: 55.0,
        device_id: "mock_device_2".to_string(),
        timestamp: timestamp,
        client_id: "test_client_1".to_string(),
    };
    let payload = Bytes::from(serde_json::to_string(&event).unwrap());

    mqtt_handler
        .handle_event("home/livingroom/temperature".to_string(), payload)
        .await;

    let conditions_data = conditions_repo.find_latest("mock_device_2").await;

    assert!(conditions_data.is_ok());

    let conditions_data = conditions_data.unwrap();
    assert!(conditions_data.is_some());

    let conditions_data = conditions_data.unwrap();

    // the latest conditions should match the event we sent
    assert_eq!(conditions_data.timestamp, timestamp);
    assert_eq!(conditions_data.temperature, "22.5");
    assert_eq!(conditions_data.humidity, "55.0");
    assert_eq!(conditions_data.device_id, "mock_device_2");
    assert_eq!(conditions_data.client_id, "test_client_1");
}
