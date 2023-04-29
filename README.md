# rust_code_camp

Sample Rust API for Power Platform

use std::collections::HashMap;
use azure_sdk_core::prelude::*;
use azure_sdk_dataverse::prelude::*;
use azure_sdk_dataverse::entity::*;
use azure_sdk_dataverse::attribute::*;

// Connect to Dataverse using authentication credentials
fn connect() -> Result<DataverseClient, Box<dyn std::error::Error>> {
    let client = DataverseClient::new(
        "<DATAPORTAL_URL>".into(),
        "<CLIENT_ID>".into(),
        "<CLIENT_SECRET>".into(),
        "<TENANT_ID>".into(),
        "<SCOPE>".into(),
    )?;

    Ok(client)
}

// Retrieve a list of entities in Dataverse
fn get_entities(client: &DataverseClient) -> Result<Vec<Entity>, Box<dyn std::error::Error>> {
    let entities = client
        .list_entities()
        .execute()?
        .entities;

    Ok(entities)
}

// Retrieve a list of attributes for a given entity in Dataverse
fn get_attributes(
    client: &DataverseClient,
    entity_name: &str,
) -> Result<Vec<Attribute>, Box<dyn std::error::Error>> {
    let attributes = client
        .get_attributes(entity_name.into())
        .execute()?
        .attributes;

    Ok(attributes)
}

// Handler function for the Rust API
fn handle_request(req: Request) -> Result<Response, Box<dyn std::error::Error>> {
    let mut headers = HashMap::new();
    headers.insert("content-type", "application/json");

    let client = connect()?;
    
    let entities = get_entities(&client)?;
    let attributes = get_attributes(&client, "account")?;
    
    let body = json!({
        "entities": entities,
        "attributes": attributes,
    });
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .headers(headers)
        .body(Body::from(serde_json::to_string(&body)?))
        .expect("Failed to build response");
    
    Ok(response)
}
