// Import reqwest crate
use reqwest::header;

// Define constants for AAD parameters
const APP_ID: &str = "your-application-id";
const CLIENT_SECRET: &str = "your-client-secret";
const TENANT_ID: &str = "your-tenant-id";
const RESOURCE_ID: &str = "https://api.powerplatform.microsoft.com";

// Define a struct for the access token response
#[derive(serde::Deserialize)]
struct TokenResponse {
access_token: String,
expires_in: u64,
token_type: String,
}

// Define a function to get an access token from AAD
async fn get_access_token() -> Result<String, Box<dyn std::error::Error>> {
// Construct the request URL for AAD token endpoint
let url = format!(
"https://login.microsoftonline.com/{}/oauth2/token",
TENANT_ID
);

// Construct the request body with the required parameters
let params = [
("grant_type", "client_credentials"),
("client_id", APP_ID),
("client_secret", CLIENT_SECRET),
("resource", RESOURCE_ID),
];

// Send a POST request and parse the JSON response
let client = reqwest::Client::new();
let response = client.post(&url).form(&params).send().await?;
let token_response: TokenResponse = response.json().await?;

// Return the access token
Ok(token_response.access_token)
}

// Define a function to call the Power platform Web API
async fn call_web_api() -> Result<(), Box<dyn std::error::Error>> {
// Get an access token from AAD
let access_token = get_access_token().await?;

// Construct the request URL for Power platform Web API endpoint
let url = format!(
"{}/api/data/v9.1/accounts?$select=name,revenue",
RESOURCE_ID
);

// Construct the authorization header with the access token
let mut headers = header::HeaderMap::new();
headers.insert(
header::AUTHORIZATION,
header::HeaderValue::from_str(&format!("Bearer {}", access_token))?,
);

// Send a GET request and print the JSON response
let client = reqwest::Client::new();
let response = client.get(&url).headers(headers).send().await?;
let json: serde_json::Value = response.json().await?;
println!("{}", json);

// Return Ok
Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
// Call the web API function
call_web_api().await?;

// Return Ok
Ok(())
}
