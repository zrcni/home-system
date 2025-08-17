use actix_web::rt;
use env_logger::Env;
use hose_service::mqtt::{create_mqtt_client, run_mqtt};
use hose_service::{configuration::Settings, startup::run};
use tokio;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings: Settings = Settings::new();
    env_logger::init_from_env(Env::default().default_filter_or(&settings.log_level));

    let (mqtt_client, _mqtt_connection) =
        create_mqtt_client(settings.mqtt_host.clone(), settings.mqtt_port);

    // these are here to be able to pass *settings* to *run*
    let conditions_db_name = settings.conditions_db_name.clone();
    let conditions_db_collection_name = settings.conditions_db_collection_name.clone();
    let conditions_mongodb_uri = settings.conditions_mongodb_uri.clone();
    let mqtt_host = settings.mqtt_host.clone();
    let mqtt_port = settings.mqtt_port;

    tokio::spawn(run_mqtt(
        mqtt_host,
        mqtt_port,
        conditions_db_name,
        conditions_db_collection_name,
        conditions_mongodb_uri,
    ));

    let server = run(settings, mqtt_client).unwrap();
    let server_handle = server.handle();

    rt::spawn(server);

    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl_c signal");

    server_handle.stop(true).await;
    Ok(())
}
