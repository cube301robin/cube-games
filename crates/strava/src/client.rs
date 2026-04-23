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

#[derive(Debug, Deserialize, Clone)]
pub struct ClubActivityAthlete {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClubActivity {
    // Strava club feed omits id and start_date for privacy
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: String,
    pub distance: f64,       // meters
    pub moving_time: u64,    // seconds — used for deduplication
    pub athlete: ClubActivityAthlete,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClubMember {
    // Strava strips id and profile from the members list for privacy
    pub firstname: String,
    pub lastname: String,
    pub membership: Option<String>, // "member" | "pending"
}

impl ClubMember {
    pub fn full_name(&self) -> String {
        format!("{} {}", self.firstname, self.lastname)
    }

    /// Same synthetic email used by the activity sync so records link up.
    pub fn synthetic_email(&self) -> String {
        format!(
            "club-{}-{}@cube.internal",
            self.firstname.to_lowercase().replace(' ', "-"),
            self.lastname.to_lowercase().replace(' ', "-"),
        )
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

    /// Fetch a page of recent club activities (up to 200 per page).
    pub async fn get_club_activities(
        &self,
        token: &str,
        club_id: u64,
        page: u32,
    ) -> anyhow::Result<Vec<ClubActivity>> {
        self.http
            .get(format!("https://www.strava.com/api/v3/clubs/{club_id}/activities"))
            .bearer_auth(token)
            .query(&[("page", page), ("per_page", 200)])
            .send()
            .await?
            .error_for_status()
            .context("strava /clubs/{id}/activities")?
            .json()
            .await
            .context("deserialize club activities")
    }

    /// Fetch all club members, paginated 30 at a time (Strava's max for this endpoint).
    pub async fn get_club_members(
        &self,
        token: &str,
        club_id: u64,
    ) -> anyhow::Result<Vec<ClubMember>> {
        let mut all = Vec::new();
        for page in 1..=100u32 {
            let batch: Vec<ClubMember> = self
                .http
                .get(format!("https://www.strava.com/api/v3/clubs/{club_id}/members"))
                .bearer_auth(token)
                .query(&[("page", page), ("per_page", 30)])
                .send()
                .await?
                .error_for_status()
                .context("strava /clubs/{id}/members")?
                .json()
                .await
                .context("deserialize club members")?;
            let done = batch.len() < 30;
            all.extend(batch);
            if done {
                break;
            }
        }
        Ok(all)
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
