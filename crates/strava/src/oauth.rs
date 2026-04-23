use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TokenExchangeResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: i64,
    pub athlete: crate::client::StravaAthlete,
}

#[derive(Clone)]
pub struct StravaOAuth {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
}

impl StravaOAuth {
    pub fn new(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
        }
    }

    /// Build the Strava OAuth authorization URL.
    pub fn authorization_url(&self) -> String {
        format!(
            "https://www.strava.com/oauth/authorize?client_id={}&response_type=code&redirect_uri={}&scope=activity:read_all&approval_prompt=auto",
            self.client_id,
            urlencoding::encode(&self.redirect_uri),
        )
    }

    /// Exchange authorization code for tokens (Strava uses a custom endpoint).
    pub async fn exchange_code(
        &self,
        code: &str,
    ) -> anyhow::Result<TokenExchangeResponse> {
        let client = reqwest::Client::new();
        let resp: TokenExchangeResponse = client
            .post("https://www.strava.com/oauth/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("code", code),
                ("grant_type", "authorization_code"),
            ])
            .send()
            .await?
            .error_for_status()
            .context("strava token exchange")?
            .json()
            .await
            .context("deserialize token response")?;
        Ok(resp)
    }

    /// Refresh an expired access token.
    pub async fn refresh_token(
        &self,
        refresh_token: &str,
    ) -> anyhow::Result<(String, String, i64)> {
        #[derive(Deserialize)]
        struct RefreshResponse {
            access_token: String,
            refresh_token: String,
            expires_at: i64,
        }
        let client = reqwest::Client::new();
        let resp: RefreshResponse = client
            .post("https://www.strava.com/oauth/token")
            .form(&[
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
                ("refresh_token", refresh_token),
                ("grant_type", "refresh_token"),
            ])
            .send()
            .await?
            .error_for_status()
            .context("strava token refresh")?
            .json()
            .await?;
        Ok((resp.access_token, resp.refresh_token, resp.expires_at))
    }
}
