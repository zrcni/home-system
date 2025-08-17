mod common;
use actix_web::test;
use common::*;
use hose_service::conditions::ConditionData;

// test health check
#[test]
async fn test_health_check() {
    let (url, _mqtt_client, _conditions_repo) = spawn_app().await;

    let res = get_health_check(&url).await;

    assert_eq!(res.status().as_u16(), 200);
    let check = res.text().await.unwrap();
    assert_eq!(check, "{\"status\":\"ok\",\"version\":\"0.1.0\"}");
}

// test health check head
#[test]
async fn test_health_check_head() {
    let (url, _mqtt_client, _conditions_repo) = spawn_app().await;

    let res = get_health_check_head(&url).await;

    assert_eq!(res.status().as_u16(), 200);
}

fn new_condition_data(device_id: &str, timestamp: i64) -> ConditionData {
    ConditionData {
        temperature: "25.0".to_string(),
        humidity: "60.0".to_string(),
        device_id: device_id.to_string(),
        timestamp: timestamp,
        client_id: "test_client".to_string(),
    }
}

// test get latest conditions by device id
#[test]
async fn test_get_latest_conditions() {
    let (url, _mqtt_client, conditions_repo) = spawn_app().await;

    let condition_data_newer = new_condition_data("mock_device", 1633072801);
    let condition_data_older = new_condition_data("mock_device", 1633072800);

    conditions_repo
        .insert_one(condition_data_older.clone())
        .await
        .expect("Failed to insert mock condition");

    conditions_repo
        .insert_one(condition_data_newer.clone())
        .await
        .expect("Failed to insert mock condition");

    let res = get_latest_conditions(&url, "mock_device").await;

    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(
        res.json::<ConditionData>().await.unwrap(),
        condition_data_newer
    );
}
