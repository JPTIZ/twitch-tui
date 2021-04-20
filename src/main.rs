use std::error::Error;
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;

use serde::{Deserialize, Serialize};

use twitch_api2::{HelixClient, helix::streams::GetStreamsRequest};
use twitch_oauth2::{AppAccessToken, ClientId, ClientSecret, Scope};

#[derive(Deserialize, Debug)]
struct Secret {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Credentials {
    access_token: String,
}

#[derive(Deserialize, Debug)]
struct CredentialsResponse {
    access_token: String,
    expires_in: isize,
    token_type: String,
}

fn load_secret<P: AsRef<Path>>(path: P) -> Result<Secret, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    Ok(serde_json::from_reader(reader).expect("Failed to deserialize JSON secret"))
}

#[tokio::main]
async fn show_result(secret: Secret) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let token =
        match AppAccessToken::get_app_access_token(
            ClientId::new(secret.client_id),
            ClientSecret::new(secret.client_secret),
            Scope::all()
        ).await {
            Ok(t) => t,
            Err(e) => panic!("App access token error: {}", e),
        };

    let client = HelixClient::new();
    let user_logins = vec![String::from("jptiz")];

    let req = GetStreamsRequest::builder()
        .user_login(user_logins)
        .build();

    let result = match client.req_get(req, &token).await {
        Ok(r) => r,
        Err(e) => panic!("Helix equest error: {}", e),
    };

    println!("Stream info:");
    println!("  Title:      {:}", &result.data[0].title);
    println!("  Viewers:    {:}", &result.data[0].viewer_count);
    println!("  Started at: {:}", &result.data[0].started_at);

    Ok(())
}

fn main() {
    println!("Hello, stream!");

    let secret = load_secret(".secret.json").expect("Failed to load secret");

    match show_result(secret) {
        Ok(_) => println!("Ok!"),
        Err(e) => panic!("Panic! {}", e),
    };
}
