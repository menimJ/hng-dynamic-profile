#[derive(Clone)]
pub struct AppConfig {
    pub port: u16,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let port = std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(8080);

        Self { port }
    }
}
