use env_logger::Env;
use hose_service::{configuration::Settings, startup::run};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Settings::new();
    env_logger::init_from_env(Env::default().default_filter_or(&settings.log_level));
    run(settings)?.await
}
