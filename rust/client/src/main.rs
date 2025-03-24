use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Request {
    client_id: String,
    request: String,
    timestamp: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    success: bool,
    server_time: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = Request {
        client_id: "uuid-1234".to_string(),
        request: "Hello".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    };

    println!("Sending request: {:?}", serde_json::to_string(&request)?);

    let response = client
        .post("http://localhost:8080")
        .json(&request)
        .send()
        .await?
        .json::<Response>()
        .await?;

    // Utilisation explicite des champs pour éviter le warning
    if response.success {
        println!("Request succeeded! Server time: {}", response.server_time);
    } else {
        println!("Request failed. Server time: {}", response.server_time);
    }

    Ok(())
}
