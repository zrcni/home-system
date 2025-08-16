use env_logger::Env;
use hose_service::mqtt::{create_mqtt_client, process_mqtt_events, setup_mqtt_subscriptions};
use hose_service::{configuration::Settings, startup::run};
use std::thread;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings: Settings = Settings::new();
    env_logger::init_from_env(Env::default().default_filter_or(&settings.log_level));

    let (mqtt_client, mqtt_connection) = create_mqtt_client(&settings);

    setup_mqtt_subscriptions(&mqtt_client);

    thread::spawn(move || {
        process_mqtt_events(mqtt_connection);
    });

    run(settings, mqtt_client)?.await
}
