use actix_cors::Cors;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, middleware, web};
use log;
use rumqttc;

use crate::configuration::Settings;
use crate::routes::*;

pub fn run(
    mut settings: Settings,
    mqtt_client: rumqttc::AsyncClient,
) -> Result<Server, std::io::Error> {
    let listener = settings.get_tcp_listener()?;
    let port = listener.local_addr().unwrap().port();
    log::info!("Listening on http://0.0.0.0:{}", port);

    let state: web::Data<AppState> = web::Data::new(AppState {
        settings: settings,
        mqtt_client: mqtt_client,
    });

    let server = HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .app_data(state.clone())
            .route("/health", web::get().to(health))
            .route("/health", web::head().to(health))
            .route("/publish", web::post().to(publish_mqtt_mock))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

pub struct AppState {
    pub settings: Settings,
    pub mqtt_client: rumqttc::AsyncClient,
}
