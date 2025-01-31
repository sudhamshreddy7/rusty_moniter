pub mod utils;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::error::Error;

async fn hit_node(node: &Node) -> Result<(), Box<dyn Error>> {
    // Create a new client
    let client = reqwest::Client::new();

    // Construct the URL
    let url = format!("http://{}:{}/logs", node.ip, node.port);

    // Set up headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", node.authorization_string))?);

    // Send the GET request
    let response = client.get(&url)
        .headers(headers)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        let body = response.text().await?;
        println!("Response from {}:{}: {}", node.ip, node.port, body);
    } else {
        println!("Error response from {}:{}: {}", node.ip, node.port, response.status());
    }

    Ok(())
}
