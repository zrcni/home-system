use env_logger::Env;
use hose_service::conditions::{MongoDBConditionsRepo, create_mongodb_conditions_repo};
use hose_service::configuration::Settings;
use hose_service::mongodb::create_mongodb_client;
use hose_service::mqtt::create_mqtt_client;
use hose_service::startup::run;

#[ctor::ctor]
fn init() {
    env_logger::init_from_env(Env::default().default_filter_or("warn"));
}

pub async fn spawn_app() -> (String, rumqttc::AsyncClient, MongoDBConditionsRepo) {
    let mut settings = Settings::new()
        .set_port(0)
        .set_conditions_db_name("test_conditions_db".to_string());

    let listener = &settings.get_tcp_listener().expect("Failed to bind address");
    let port = listener.local_addr().unwrap().port();

    let (mqtt_client, _mqtt_eventloop) =
        create_mqtt_client(settings.mqtt_host.clone(), settings.mqtt_port);

    let mongo_client = create_mongodb_client(settings.conditions_mongodb_uri.clone())
        .await
        .expect("Failed to create MongoDB client");

    let conditions_repo = create_mongodb_conditions_repo(
        mongo_client,
        settings.conditions_db_name.clone(),
        settings.conditions_db_collection_name.clone(),
    )
    .expect("Failed to create MongoDBConditionsRepo");

    let app = run(settings, mqtt_client.clone(), conditions_repo.clone())
        .expect("Failed to bind address");
    let _ = actix_web::rt::spawn(app);
    (
        format!("http://127.0.0.1:{}", port),
        mqtt_client,
        conditions_repo,
    )
}

pub async fn get_health_check(address: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/health", address))
        .send()
        .await
        .unwrap()
}

pub async fn get_health_check_head(address: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .head(format!("{}/health", address))
        .send()
        .await
        .unwrap()
}

pub async fn get_latest_conditions(address: &str, device_id: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    client
        .get(format!("{}/conditions/{}/latest", address, device_id))
        .send()
        .await
        .unwrap()
}
