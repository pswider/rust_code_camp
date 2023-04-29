# Unleashing the Power of Rust: Crafting High-Performance Code for AI-infused apps

Session Guide and Notes

```
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
    
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");

}
```



## Oauth

To connect to an API in Azure using OAuth 2.0 and App Registration, you'll need to follow these steps:

1. Register your application in Azure Active Directory (AAD).
2. Grant your application the necessary permissions to access the target API.
3. Acquire an access token using the OAuth 2.0 client credentials flow.
4. Use the access token to call the target API.

The following example demonstrates how to use the `reqwest` and `serde` crates in Rust to connect to an API in Azure using OAuth 2.0 and App Registration:

First, add the required dependencies in your `Cargo.toml`:

```
tomlCopy code[dependencies]
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
```

Next, use the following code as a starting point:

```
rustCopy codeuse reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Replace these values with your own
    let tenant_id = "YOUR_TENANT_ID";
    let client_id = "YOUR_CLIENT_ID";
    let client_secret = "YOUR_CLIENT_SECRET";
    let api_url = "YOUR_API_URL";

    let token_url = format!(
        "https://login.microsoftonline.com/{}/oauth2/v2.0/token",
        tenant_id
    );

    let token_params = [
        ("grant_type", "client_credentials"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("scope", "https://graph.microsoft.com/.default"),
    ];

    let token_response: TokenResponse = client
        .post(&token_url)
        .form(&token_params)
        .send()
        .await?
        .json()
        .await?;

    let response = client
        .get(api_url)
        .bearer_auth(token_response.access_token)
        .send()
        .await?;

    println!("Response: {:?}", response);

    Ok(())
}
```

Replace `YOUR_TENANT_ID`, `YOUR_CLIENT_ID`, `YOUR_CLIENT_SECRET`, and `YOUR_API_URL` with the appropriate values from your Azure App Registration and target API. This example uses the Microsoft Graph API as the target API, but you can replace the scope and API URL with the values for your specific API.

This code demonstrates how to:

- Acquire an access token using the client credentials flow.
- Use the access token as a Bearer token to call the target API.
- Print the response from the API.

Don't forget to run `cargo run` to execute the code. If you encounter any issues, make sure your App Registration is configured correctly and you have the necessary permissions to access the target API.