use std::net::TcpListener;

pub struct Settings {
    pub log_level: String,
    pub port: u16,
    pub mqtt_host: String,
    pub mqtt_port: u16,
    pub conditions_mongodb_uri: String,
    pub conditions_db_name: String,
    pub conditions_db_collection_name: String,
    tcp_listener: Option<TcpListener>,
}

// @todo get from env variables
impl Settings {
    pub fn new() -> Self {
        Settings {
            mqtt_host: std::env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string()),
            mqtt_port: std::env::var("MQTT_PORT")
                .unwrap_or_else(|_| "1884".to_string())
                .parse()
                .unwrap_or(1884),
            conditions_mongodb_uri: std::env::var("CONDITIONS_DB_URI")
                .unwrap_or_else(|_| "mongodb://localhost:27017".into()),
            conditions_db_name: std::env::var("CONDITIONS_DB_NAME")
                .unwrap_or_else(|_| "conditions_db".to_string()),
            conditions_db_collection_name: std::env::var("CONDITIONS_DB_COLLECTION_NAME")
                .unwrap_or_else(|_| "conditions_data".to_string()),
            // Default log level is info, can be overridden by LOG_LEVEL env variable
            log_level: std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string()),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3002".to_string())
                .parse()
                .unwrap_or(3002),
            tcp_listener: None,
        }
    }

    pub fn set_port(mut self, port: u16) -> Self {
        self.port = port;
        return self;
    }

    pub fn set_log_level(mut self, log_level: String) -> Self {
        self.log_level = log_level;
        return self;
    }

    pub fn get_tcp_listener(&mut self) -> std::io::Result<TcpListener> {
        if let Some(listener) = &self.tcp_listener {
            return Ok(listener.try_clone()?);
        } else {
            let listener = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
            self.tcp_listener = Some(listener.try_clone()?);
            return Ok(listener);
        }
    }
}
