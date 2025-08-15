use std::net::TcpListener;

pub struct Settings {
    pub log_level: String,
    pub port: u16,
    tcp_listener: Option<TcpListener>,
}

impl Settings {
    pub fn new() -> Self {
        Settings {
            log_level: "info".to_string(),
            port: 3002,
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
