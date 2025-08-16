use rumqttc::{Client, Connection, Event, Incoming, MqttOptions, SubscribeFilter};
use std::time::Duration;

use crate::configuration::Settings;
use crate::mqtt_handlers::handle_incoming_mqtt_event;

pub struct MqttTopics;

impl MqttTopics {
    pub const LIVING_ROOM_CONDITIONS_UPDATED: &'static str = "home/livingroom/temperature";
}

pub fn create_mqtt_client(settings: &Settings) -> (Client, Connection) {
    let mut mqttoptions: MqttOptions = MqttOptions::new(
        "hose-service",
        settings.mqtt_host.clone(),
        settings.mqtt_port,
    );
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    let (client, connection) = Client::new(mqttoptions, 10);
    (client, connection)
}

pub fn process_mqtt_events(mut connection: Connection) {
    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notif) => match notif {
                Event::Incoming(Incoming::PubAck(puback)) => {
                    println!("{i}. Received PubAck: {:?}", puback);
                }
                Event::Incoming(Incoming::Publish(publish)) => {
                    handle_incoming_mqtt_event(publish);
                }
                _ => {}
            },
            Err(error) => {
                println!("{i}. Notification = {error:?}");
            }
        }
    }
}

pub fn setup_mqtt_subscriptions(client: &Client) {
    let topics = vec![(
        MqttTopics::LIVING_ROOM_CONDITIONS_UPDATED.to_string(),
        rumqttc::QoS::AtLeastOnce,
    )];

    let topics: Vec<SubscribeFilter> = topics
        .into_iter()
        .map(|(path, qos)| SubscribeFilter { path, qos })
        .collect();

    client.subscribe_many(topics).unwrap();
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
