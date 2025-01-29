
use dotenv::dotenv;
use notify::Error;
use std::env;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio;

pub fn get_api_key() -> String {
    dotenv().ok(); // Loads .env variables
    let api_key = env::var("GEMINI_API_KEY").expect("API_KEY not set");
    api_key // Return the API_KEY
}

pub fn get_api_url() -> String {
    dotenv().ok(); // Loads .env variables
    let api_key = env::var("GEMINI_API_URL").expect("API_URL not set");
    api_key // Return the API_KEY
}


// Define the structure of the request payload
#[derive(Debug, Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
}

#[derive(Debug, Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Debug, Serialize)]
struct Part {
    text: String,
}

// Define the structure of the response (using serde_json::Value for flexibility)
#[derive(Debug, Deserialize)]
struct GenerateContentResponse {
    // Define fields based on actual response structure
    // For demonstration, we'll use serde_json::Value
    // Replace with specific fields as needed
    #[serde(flatten)]
    data: serde_json::Value,
}

async fn generate_response(content: String) -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env
    dotenv().ok();

    // Retrieve the API key from environment variables
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set in .env");

    // Initialize the HTTP client
    let client = Client::new();

    // Define the API endpoint with the API key as a query parameter
    let url = "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent";

    // Construct the full URL with the API key
    let request_url = format!("{}?key={}", url, api_key);

    // Create the request payload
    let request_payload = GenerateContentRequest {
        contents: vec![
            Content {
                parts: vec![
                    Part {
                        text: content,
                    },
                ],
            },
        ],
    };

    // Send the POST request
    let response = client
        .post(&request_url)
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await?;

    // Check if the request was successful
    if response.status().is_success() {
        // Deserialize the response
        let response_data: GenerateContentResponse = response.json().await?;
        println!("Response: {:#?}", response_data.data);
    } else {
        // Handle errors
        let error_text = response.text().await?;
        //eprintln!("Error: HTTP {} - {}", response.status(), error_text);
    }

    Ok(())
}

#[tauri::command]
pub async fn call_api(content: String){
    if let Err(err) = generate_response(content).await {
        eprintln!("API Call Error: {}", err);
    }
}