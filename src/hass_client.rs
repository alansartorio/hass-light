use reqwest::{Client, Result};
use serde_json::Value;

pub struct HassClient {
    host: String,
    port: usize,
    token: String,
    http_client: Client,
}

impl HassClient {
    pub fn new(host: String, port: usize, token: String) -> Self {
        Self {
            host,
            port,
            token,
            http_client: Client::new(),
        }
    }

    pub async fn get_state(&self, entity_id: &str) -> Result<Value> {
        let HassClient {
            host, port, token, ..
        } = self;

        self.http_client
            .get(format!("{host}:{port}/api/states/{entity_id}"))
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await?
            .text()
            .await
            .map(|t| serde_json::from_str(&t).unwrap())
    }

    pub async fn set_state(&self, domain: &str, service: &str, value: Value) -> Result<Value> {
        let HassClient {
            host, port, token, ..
        } = self;

        self.http_client
            .post(format!("{host}:{port}/api/services/{domain}/{service}"))
            .json(&value)
            .header("Authorization", format!("Bearer {token}"))
            .send()
            .await?
            .json()
            .await
    }
}
