use anyhow::Context;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StravaActivity {
    pub id: u64,
    pub name: String,
    pub distance: f64, // meters
    #[serde(rename = "type")]
    pub activity_type: String, // "Run", "Walk", etc.
    pub start_date: String,     // ISO 8601 UTC
    pub start_date_local: String,
    pub athlete: AthleteRef,
}

impl StravaActivity {
    /// Distance in kilometres.
    pub fn distance_km(&self) -> f64 {
        self.distance / 1000.0
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AthleteRef {
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct StravaAthlete {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub username: Option<String>,
    pub profile: Option<String>,
    pub email: Option<String>,
}

impl StravaAthlete {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }
}

#[derive(Clone)]
pub struct StravaClient {
    http: Client,
}

impl StravaClient {
    pub fn new() -> Self {
        Self {
            http: Client::builder()
                .user_agent("CubeGames/2026")
                .build()
                .expect("http client"),
        }
    }

    pub async fn get_athlete(&self, token: &str) -> anyhow::Result<StravaAthlete> {
        self.http
            .get("https://www.strava.com/api/v3/athlete")
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()
            .context("strava /athlete")?
            .json()
            .await
            .context("deserialize athlete")
    }

    pub async fn get_activity(&self, token: &str, id: u64) -> anyhow::Result<StravaActivity> {
        self.http
            .get(format!("https://www.strava.com/api/v3/activities/{id}"))
            .bearer_auth(token)
            .send()
            .await?
            .error_for_status()
            .context("strava /activities/{id}")?
            .json()
            .await
            .context("deserialize activity")
    }
}

impl Default for StravaClient {
    fn default() -> Self {
        Self::new()
    }
}
