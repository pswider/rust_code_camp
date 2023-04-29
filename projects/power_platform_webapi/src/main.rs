use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::thread;
use tiny_http::{Response, Server};

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
}

#[derive(Serialize, Deserialize)]
struct Lead {
    name: String,
    // Add other lead fields here
}

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        let client = Client::new();

        // Replace these values with your own
        let tenant_id = "5f4520b5-b9f7-4e21-a41b-e0610de7f42d";
        let client_id = "6b67080c-710e-4c34-9faf-da2de0b43304";
        let client_secret = "MlB8Q~ZRDnGJ.fSdFTSU4envbsPDqSdspD7Q2bAF";
        let api_url = "https://radev-11-3.crm.dynamics.com/";

        let auth_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/authorize?response_type=code&client_id={}&redirect_uri=http://localhost:8080&scope=https://radev-11-3.crm.dynamics.com/.default",
            tenant_id, client_id
        );

        println!("Open this URL in your browser to authenticate:\n{}", auth_url);

        let server = Server::http("localhost:8080").unwrap();
        let request = server.recv().unwrap();

        let request_url = request.url().to_string();

        let response = Response::from_string("You can close this window now.");
        request.respond(response).unwrap();

        let mut code = String::new();
        for param in request_url.split('?').nth(1).unwrap().split('&') {
            let (key, value) = param.split_once('=').unwrap();
            if key == "code" {
                code = value.to_string();
                break;
            }
        }

        let token_url = format!(
            "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
            tenant_id
        );

        let token_params = [
            ("grant_type", "authorization_code"),
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("redirect_uri", "http://localhost:8080"),
            ("code", &code),
        ];

        let token_response: TokenResponse = client
            .post(&token_url)
            .form(&token_params)
            .send()
            .await?
            .json()
            .await?;

        let response = client
            .get(format!("{}/api/data/v9.0/leads", api_url))
            .bearer_auth(&token_response.access_token)
            .send()
            .await?;

        let leads: Vec<Lead> = response.json().await?;

        for lead in leads {
            println!("Title: {}", lead.name);
        }

        Ok(())
    })
}
/* 
fn get_azure_app_reg(tenant_id, client_id, client_secret, api_url) -> () {
    // Replace these values with your own
    let tenant_id = "5f4520b5-b9f7-4e21-a41b-e0610de7f42d";
    let client_id = "6b67080c-710e-4c34-9faf-da2de0b43304";
    let client_secret = "MlB8Q~ZRDnGJ.fSdFTSU4envbsPDqSdspD7Q2bAF";
    let api_url = "http://localhost:8080";
    (tenant_id, client_id, client_secret, api_url)
} */