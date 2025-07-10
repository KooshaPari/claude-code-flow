use anyhow::{anyhow, Result};
use base64::Engine as _;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};
use url::Url;

/// OAuth application configuration
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub scope: String,
}

impl OAuthConfig {
    /// Create OAuth config from environment variables
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            client_id: std::env::var("GITHUB_CLIENT_ID")
                .map_err(|_| anyhow!("GITHUB_CLIENT_ID not found"))?,
            client_secret: std::env::var("GITHUB_CLIENT_SECRET")
                .map_err(|_| anyhow!("GITHUB_CLIENT_SECRET not found"))?,
            redirect_uri: std::env::var("GITHUB_REDIRECT_URI")
                .unwrap_or_else(|_| "http://localhost:8080/callback".to_string()),
            scope: std::env::var("GITHUB_SCOPE")
                .unwrap_or_else(|_| "repo,user,workflow".to_string()),
        })
    }
}

/// GitHub authentication manager
#[derive(Debug)]
pub struct GitHubAuth {
    config: OAuthConfig,
    client: Client,
}

impl GitHubAuth {
    /// Create new authentication manager
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    /// Generate OAuth authorization URL
    pub fn generate_auth_url(&self, state: Option<&str>) -> Result<String> {
        let mut url = Url::parse("https://github.com/login/oauth/authorize")?;
        
        let mut params = vec![
            ("client_id", self.config.client_id.as_str()),
            ("redirect_uri", self.config.redirect_uri.as_str()),
            ("scope", self.config.scope.as_str()),
        ];
        
        if let Some(state) = state {
            params.push(("state", state));
        }
        
        url.query_pairs_mut().extend_pairs(&params);
        
        debug!("Generated OAuth URL: {}", url.as_str());
        Ok(url.to_string())
    }

    /// Exchange authorization code for access token
    pub async fn exchange_code_for_token(&self, code: &str, state: Option<&str>) -> Result<AccessTokenResponse> {
        info!("Exchanging authorization code for access token");
        
        let mut params = HashMap::new();
        params.insert("client_id", &self.config.client_id);
        params.insert("client_secret", &self.config.client_secret);
        params.insert("code", code);
        
        if let Some(state) = state {
            params.insert("state", state);
        }
        
        let response = self.client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to exchange code: {}", response.status()));
        }
        
        let token_response: AccessTokenResponse = response.json().await?;
        
        if let Some(ref error) = token_response.error {
            return Err(anyhow!("OAuth error: {}", error));
        }
        
        info!("Successfully obtained access token");
        Ok(token_response)
    }

    /// Refresh access token using refresh token
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AccessTokenResponse> {
        info!("Refreshing access token");
        
        let params = [
            ("client_id", &self.config.client_id),
            ("client_secret", &self.config.client_secret),
            ("grant_type", &"refresh_token".to_string()),
            ("refresh_token", &refresh_token.to_string()),
        ];
        
        let response = self.client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to refresh token: {}", response.status()));
        }
        
        let token_response: AccessTokenResponse = response.json().await?;
        
        if let Some(ref error) = token_response.error {
            return Err(anyhow!("OAuth error: {}", error));
        }
        
        info!("Successfully refreshed access token");
        Ok(token_response)
    }

    /// Revoke access token
    pub async fn revoke_token(&self, token: &str) -> Result<()> {
        info!("Revoking access token");
        
        let auth_header = format!("Basic {}", 
            base64::engine::general_purpose::STANDARD.encode(
                format!("{}:{}", self.config.client_id, self.config.client_secret)
            )
        );
        
        let params = [("access_token", token)];
        
        let response = self.client
            .delete("https://api.github.com/applications/grants")
            .header("Authorization", auth_header)
            .form(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to revoke token: {}", response.status()));
        }
        
        info!("Successfully revoked access token");
        Ok(())
    }

    /// Validate access token
    pub async fn validate_token(&self, token: &str) -> Result<TokenValidation> {
        debug!("Validating access token");
        
        let response = self.client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "Claude-Flow-GitHub-Integration")
            .send()
            .await?;
        
        if response.status().is_success() {
            let user: GitHubUser = response.json().await?;
            Ok(TokenValidation {
                valid: true,
                user: Some(user),
                scopes: self.extract_scopes_from_response(&response),
            })
        } else if response.status() == 401 {
            Ok(TokenValidation {
                valid: false,
                user: None,
                scopes: Vec::new(),
            })
        } else {
            Err(anyhow!("Failed to validate token: {}", response.status()))
        }
    }

    /// Extract scopes from response headers
    fn extract_scopes_from_response(&self, response: &reqwest::Response) -> Vec<String> {
        response
            .headers()
            .get("x-oauth-scopes")
            .and_then(|header| header.to_str().ok())
            .map(|scopes| {
                scopes
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Get user information from token
    pub async fn get_user_info(&self, token: &str) -> Result<GitHubUser> {
        debug!("Getting user information");
        
        let response = self.client
            .get("https://api.github.com/user")
            .header("Authorization", format!("token {}", token))
            .header("User-Agent", "Claude-Flow-GitHub-Integration")
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to get user info: {}", response.status()));
        }
        
        Ok(response.json().await?)
    }

    /// Check if token has required scopes
    pub async fn check_scopes(&self, token: &str, required_scopes: &[&str]) -> Result<bool> {
        debug!("Checking token scopes");
        
        let validation = self.validate_token(token).await?;
        
        if !validation.valid {
            return Ok(false);
        }
        
        let has_all_scopes = required_scopes.iter().all(|required| {
            validation.scopes.iter().any(|scope| scope == required)
        });
        
        if !has_all_scopes {
            warn!("Token missing required scopes. Required: {:?}, Available: {:?}", 
                  required_scopes, validation.scopes);
        }
        
        Ok(has_all_scopes)
    }

    /// Create device flow for CLI authentication
    pub async fn device_flow(&self) -> Result<DeviceCodeResponse> {
        info!("Starting device flow authentication");
        
        let params = [
            ("client_id", &self.config.client_id),
            ("scope", &self.config.scope),
        ];
        
        let response = self.client
            .post("https://github.com/login/device/code")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await?;
        
        if !response.status().is_success() {
            return Err(anyhow!("Failed to start device flow: {}", response.status()));
        }
        
        Ok(response.json().await?)
    }

    /// Poll for device flow completion
    pub async fn poll_device_flow(&self, device_code: &str) -> Result<AccessTokenResponse> {
        info!("Polling for device flow completion");
        
        let params = [
            ("client_id", &self.config.client_id),
            ("device_code", &device_code.to_string()),
            ("grant_type", &"urn:ietf:params:oauth:grant-type:device_code".to_string()),
        ];
        
        let response = self.client
            .post("https://github.com/login/oauth/access_token")
            .header("Accept", "application/json")
            .form(&params)
            .send()
            .await?;
        
        let token_response: AccessTokenResponse = response.json().await?;
        
        if let Some(ref error) = token_response.error {
            match error.as_str() {
                "authorization_pending" => return Err(anyhow!("Authorization pending")),
                "slow_down" => return Err(anyhow!("Slow down")),
                "expired_token" => return Err(anyhow!("Device code expired")),
                "access_denied" => return Err(anyhow!("Access denied")),
                _ => return Err(anyhow!("OAuth error: {}", error)),
            }
        }
        
        info!("Device flow completed successfully");
        Ok(token_response)
    }
}

/// Access token response from GitHub OAuth
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: Option<String>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub refresh_token: Option<String>,
    pub refresh_token_expires_in: Option<u64>,
    pub error: Option<String>,
    pub error_description: Option<String>,
}

/// Device code response for device flow
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

/// Token validation result
#[derive(Debug)]
pub struct TokenValidation {
    pub valid: bool,
    pub user: Option<GitHubUser>,
    pub scopes: Vec<String>,
}

/// GitHub user information
#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub name: Option<String>,
    pub email: Option<String>,
    pub avatar_url: String,
    pub html_url: String,
    pub company: Option<String>,
    pub location: Option<String>,
    pub bio: Option<String>,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
}

/// Token storage interface
pub trait TokenStorage {
    fn store_token(&self, token: &str) -> Result<()>;
    fn load_token(&self) -> Result<Option<String>>;
    fn remove_token(&self) -> Result<()>;
}

/// File-based token storage
pub struct FileTokenStorage {
    path: std::path::PathBuf,
}

impl FileTokenStorage {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}

impl TokenStorage for FileTokenStorage {
    fn store_token(&self, token: &str) -> Result<()> {
        std::fs::write(&self.path, token)?;
        Ok(())
    }
    
    fn load_token(&self) -> Result<Option<String>> {
        match std::fs::read_to_string(&self.path) {
            Ok(token) => Ok(Some(token.trim().to_string())),
            Err(_) => Ok(None),
        }
    }
    
    fn remove_token(&self) -> Result<()> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oauth_config_creation() {
        let config = OAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
            scope: "repo,user".to_string(),
        };
        
        assert_eq!(config.client_id, "test_id");
        assert_eq!(config.scope, "repo,user");
    }

    #[test]
    fn test_auth_url_generation() {
        let config = OAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:8080/callback".to_string(),
            scope: "repo,user".to_string(),
        };
        
        let auth = GitHubAuth::new(config);
        let url = auth.generate_auth_url(Some("test_state")).unwrap();
        
        assert!(url.contains("client_id=test_id"));
        assert!(url.contains("state=test_state"));
        assert!(url.contains("scope=repo%2Cuser"));
    }
}