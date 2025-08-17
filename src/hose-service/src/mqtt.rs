use crate::{
    conditions::MongoDBConditionsRepo,
    mqtt_handlers::{MqttHandler, create_new_mqtt_handler},
};
use log;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, SubscribeFilter};
use std::time::Duration;

pub struct MqttTopics;

impl MqttTopics {
    pub const LIVING_ROOM_CONDITIONS_UPDATED: &'static str = "home/livingroom/temperature";
}

pub fn create_mqtt_client(host: String, port: u16) -> (AsyncClient, EventLoop) {
    let mut mqttoptions: MqttOptions = MqttOptions::new("hose-service", host.to_string(), port);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, eventloop) = AsyncClient::new(mqttoptions, 10);
    (client, eventloop)
}

pub async fn process_mqtt_events(mut eventloop: EventLoop, mqtt_handler: MqttHandler) {
    while let Ok(notification) = eventloop.poll().await {
        match notification {
            Event::Incoming(Incoming::PubAck(puback)) => {
                log::debug!("Received PubAck: {:?}", puback);
            }
            Event::Incoming(Incoming::Publish(publish)) => {
                mqtt_handler
                    .handle_event(publish.topic, publish.payload)
                    .await;
            }
            _ => {}
        }
    }
}

pub async fn setup_mqtt_subscriptions(client: AsyncClient) {
    let topics = vec![(
        MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED.to_string(),
        rumqttc::QoS::AtLeastOnce,
    )];

    let topics: Vec<SubscribeFilter> = topics
        .into_iter()
        .map(|(path, qos)| SubscribeFilter { path, qos })
        .collect();

    client.subscribe_many(topics).await.unwrap();
}

// fn handle_mqtt_event(incoming_event: Incoming) {
// }

// {
//     humidity: 20.1,
//     temperature: 23.2,
//     device_id: 'mock_device',
//     timestamp: 1665779082868,
//     client_id: 'mock_client',
// }

pub async fn run_mqtt(
    mqtt_client: AsyncClient,
    mqtt_connection: EventLoop,
    conditions_repo: MongoDBConditionsRepo,
) {
    let mqtt_handler = create_new_mqtt_handler(conditions_repo);
    setup_mqtt_subscriptions(mqtt_client).await;
    log::info!("MQTT event processing starting");
    process_mqtt_events(mqtt_connection, mqtt_handler).await;
}
