
use std::sync::mpsc;
use tauri::Window;
use url::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub scope: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OAuthConfigs {
    pub google: OAuthConfig,
    pub github: OAuthConfig,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub email: String,
    pub avatar: Option<String>,
    pub provider: String,
    pub access_token: String,
}

#[tauri::command]
pub async fn login_with_provider(_window: Window, provider: String) -> Result<UserInfo, String> {
    let configs = load_oauth_configs()?;

    // Get provider-specific configuration
    let config = match provider.as_str() {
        "google" => configs.google,
        "github" => configs.github,
        _ => return Err(format!("Unsupported provider: {}", provider)),
    };

    // OAuth configuration for the server
    let oauth_config = tauri_plugin_oauth::OauthConfig {
        ports: Some(vec![8000, 8001, 8002]),
        response: Some("OAuth process completed. You can close this window.".into()),
    };

    // Create a channel to receive the authorization code
    let (tx, rx) = mpsc::channel::<String>();
    let tx_clone = tx.clone();

    // Start the OAuth server
    let port = tauri_plugin_oauth::start_with_config(oauth_config, move |url| {
        // Extract the authorization code from the URL
        let url_obj = Url::parse(&url).expect("Failed to parse URL");
        let code = url_obj.query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .expect("No code found in URL");

        // Send the code through the channel
        tx_clone.send(code).expect("Failed to send code");
    })
    .map_err(|err| err.to_string())?;

    // Build the authorization URL
    let mut auth_url_obj = Url::parse(&config.auth_url).map_err(|err| err.to_string())?;
    auth_url_obj.query_pairs_mut()
        .append_pair("client_id", &config.client_id)
        .append_pair("redirect_uri", &format!("http://localhost:{}", port))
        .append_pair("scope", &config.scope)
        .append_pair("response_type", "code");

    // Generate a random state for CSRF protection
    let state = generate_random_string(16);
    auth_url_obj.query_pairs_mut().append_pair("state", &state);

    // Open the authorization URL in the default browser
    tauri_plugin_opener::open_url(auth_url_obj.as_str(), None::<&str>)
        .map_err(|err| err.to_string())?;

    // Wait for the authorization code
    let code = rx.recv().map_err(|err| err.to_string())?;

    // Exchange the code for an access token
    let client = reqwest::Client::new();
    let token_response = client.post(&config.token_url)
        .form(&[
            ("client_id", config.client_id),
            ("client_secret", config.client_secret),
            ("code", code),
            ("redirect_uri", format!("http://localhost:{}", port)),
            ("grant_type", "authorization_code".to_string()),
        ])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|err| err.to_string())?;

        print!( "token_response: {:?}",token_response);
    if !token_response.status().is_success() {
        return Err(format!("Failed to exchange code for token: {}", token_response.status()));
    }

    let token_data: serde_json::Value = token_response.json().await.map_err(|err| err.to_string())?;
    let access_token = token_data["access_token"].as_str().ok_or("No access token found")?;

    print!( "access_token: {:?}",access_token);

    // Get user info
    let user_info_response = match provider.as_str() {
        "google" => client.get(&config.user_info_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/json")
            .send()
            .await,
        "github" => client.get(&config.user_info_url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", "tauri-app") 
            .send()
            .await,
        _ => return Err(format!("Unsupported provider: {}", provider)),
    }
    .map_err(|err| err.to_string())?;

    if !user_info_response.status().is_success() {
        return Err(format!("Failed to get user info: {}", user_info_response.status()));
    }

    let user_info: serde_json::Value = user_info_response.json().await.map_err(|err| err.to_string())?;

    // Extract user info based on provider
    let (id, name, email, avatar) = match provider.as_str() {
        "google" => (
            user_info["sub"].as_str().unwrap_or("").to_string(),
            user_info["name"].as_str().unwrap_or("").to_string(),
            user_info["email"].as_str().unwrap_or("").to_string(),
            user_info["picture"].as_str().map(|s| s.to_string()),
        ),
        "github" => (
            user_info["id"].to_string(),
            user_info["name"].as_str().unwrap_or_else(|| user_info["login"].as_str().unwrap_or("")).to_string(),
            user_info["email"].as_str().unwrap_or("").to_string(),
            user_info["avatar_url"].as_str().map(|s| s.to_string()),
        ),
        _ => return Err(format!("Unsupported provider: {}", provider)),
    };

    Ok(UserInfo {
        id,
        name,
        email,
        avatar,
        provider,
        access_token: access_token.to_string(),
    })
}

// Helper function to generate a random string
fn generate_random_string(length: usize) -> String {
    use rand::{Rng, thread_rng};
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = thread_rng();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn load_oauth_configs() -> Result<OAuthConfigs, String> {
    let config_path = std::path::Path::new("oauth_config.json");

    if config_path.exists() {
        let config_content = std::fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        let configs: OAuthConfigs = serde_json::from_str(&config_content)
            .map_err(|e| format!("Failed to parse config file: {}", e))?;

        return Ok(configs);
    }

    Err("cannot find config".to_string())
}

