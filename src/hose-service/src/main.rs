use actix_web::rt;
use env_logger::Env;
use hose_service::conditions::create_mongodb_conditions_repo;
use hose_service::mongodb::create_mongodb_client;
use hose_service::mqtt::{create_mqtt_client, run_mqtt};
use hose_service::{configuration::Settings, startup::run};
use tokio;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings: Settings = Settings::new();
    env_logger::init_from_env(Env::default().default_filter_or(&settings.log_level));

    let (mqtt_client, mqtt_eventloop) =
        create_mqtt_client(settings.mqtt_host.clone(), settings.mqtt_port);

    // these are here to be able to pass *settings* to *run*
    let conditions_db_name = settings.conditions_db_name.clone();
    let conditions_db_collection_name = settings.conditions_db_collection_name.clone();
    let conditions_mongodb_uri = settings.conditions_mongodb_uri.clone();

    let mongo_client = create_mongodb_client(conditions_mongodb_uri)
        .await
        .expect("Failed to create MongoDB client");

    tokio::spawn(run_mqtt(
        mqtt_client.clone(),
        mqtt_eventloop,
        create_mongodb_conditions_repo(
            mongo_client,
            conditions_db_name,
            conditions_db_collection_name,
        )
        .expect("Failed to create ConditionsRepo"),
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
