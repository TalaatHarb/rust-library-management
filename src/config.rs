
#[derive(Clone)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
    pub oauth2_schema: String,
    pub oauth2_host: String,
    pub oauth2_port: u16,
    pub oauth2_realm: String,
    pub oauth2_issuer_url: String,
    pub oauth2_token_url: String,
}

impl Config {
    pub fn from_env() -> Self {
        let oauth2_schema = std::env::var("OAUTH2_SCHEMA")
            .unwrap_or_else(|_| "http".to_string());
        let oauth2_host = std::env::var("OAUTH2_HOST")
            .unwrap_or_else(|_| "localhost".to_string());
        let oauth2_port = std::env::var("OAUTH2_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080u16);
        let oauth2_realm = std::env::var("OAUTH2_REALM")
            .unwrap_or_else(|_| "master".to_string());

        let default_base = format!(
            "{}://{}:{}/realms/{}",
            oauth2_schema, oauth2_host, oauth2_port, oauth2_realm
        );

        let oauth2_issuer_url = std::env::var("OAUTH2_ISSUER_URL")
            .unwrap_or_else(|_| default_base.clone());
        let oauth2_token_url = std::env::var("OAUTH2_TOKEN_URL")
            .unwrap_or_else(|_| format!("{}/protocol/openid-connect/token", default_base));

        Self {
            port: std::env::var("PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(3000u16),
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| {
                    "postgres://postgres:postgres@localhost/library_db".to_string()
                }),
            oauth2_schema,
            oauth2_host,
            oauth2_port,
            oauth2_realm,
            oauth2_issuer_url,
            oauth2_token_url,
        }
    }
}
