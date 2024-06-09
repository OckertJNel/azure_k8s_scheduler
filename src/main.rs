use reqwest::Client;
use std::error::Error;
use std::fs;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    println!("Azure K8S Scheduler Starting...");
    println!("Reading config file...");
    
    match read_config_file() {
        Ok(config) => {
            println!("Config file read successfully.");
            println!("Client ID: {}", config.client_id);
            println!("Client Secret: {}", config.client_secret);
            println!("Token Endpoint: {}", config.token_endpoint);
            println!("Timeout Seconds: {}", config.timeout_seconds);
            println!("Getting access token...");
            
            match get_access_token(&config.token_endpoint, &config.client_id, &config.client_secret, "common").await {
                Ok(access_token) => {
                    println!("Access token: {}", access_token);
                },
                Err(e) => {
                    println!("Error getting access token: {}", e);
                }
            }
            
        },
        Err(e) => {
            println!("Error reading config file: {}", e);
        }
    }
}

async fn get_access_token(token_endpoint: &str, client_id:&str,client_secret:&str,tenant_id:&str) -> Result<String, Box<dyn Error>> {
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("grant_type", "client_credentials"),
        ("scope", "https://management.azure.com/.default")
    ];
    
    let client = Client::new();
    
    let response = client.post(&*token_endpoint)
        .form(&params)
        .send()
        .await?;
    
    let body = response.text().await?;
    
    let json : serde_json::Value = serde_json::from_str(&body)?; 
    
    let access_token = json["access_token"].as_str().unwrap_or_default().to_string();
    
    Ok(access_token)
}

#[derive(Deserialize)]
struct AppConfig {
    pub client_id: String,
    pub client_secret: String,
    pub token_endpoint: String,
    pub timeout_seconds: u64,
}

fn read_config_file() -> Result<AppConfig, Box<dyn Error>> {
    let contents = fs::read_to_string("config.json")?;
    let config: AppConfig = serde_json::from_str(&contents)?;
    
    Ok(config)
}