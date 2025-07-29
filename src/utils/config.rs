use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub mcp_api_url: String,
    pub mcp_api_key: Option<String>,
    pub lnd_host: String,
    pub lnd_port: u16,
    pub lnd_macaroon_path: String,
    pub lnd_tls_cert_path: String,
    pub server_port: u16,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            mcp_api_url: "https://api.dazno.de".to_string(),
            mcp_api_key: None,
            lnd_host: "localhost".to_string(),
            lnd_port: 10009,
            lnd_macaroon_path: "/lnd/admin.macaroon".to_string(),
            lnd_tls_cert_path: "/lnd/tls.cert".to_string(),
            server_port: 3000,
        }
    }
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            mcp_api_url: env::var("MCP_API_URL")
                .unwrap_or_else(|_| "https://api.dazno.de".to_string()),
            mcp_api_key: env::var("MCP_API_KEY").ok(),
            lnd_host: env::var("LND_HOST")
                .unwrap_or_else(|_| "localhost".to_string()),
            lnd_port: env::var("LND_PORT")
                .unwrap_or_else(|_| "10009".to_string())
                .parse()
                .unwrap_or(10009),
            lnd_macaroon_path: env::var("LND_MACAROON_PATH")
                .unwrap_or_else(|_| "/lnd/admin.macaroon".to_string()),
            lnd_tls_cert_path: env::var("LND_TLS_CERT_PATH")
                .unwrap_or_else(|_| "/lnd/tls.cert".to_string()),
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap_or(3000),
        }
    }
}