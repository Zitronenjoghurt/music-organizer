use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, serde::Deserialize)]
struct AcoustIDResponse {
    results: Vec<AcoustIDResult>,
}

#[derive(Debug, serde::Deserialize)]
struct AcoustIDResult {
    #[serde(default)]
    recordings: Vec<AcoustIDRecording>,
    score: f64,
}

#[derive(Debug, serde::Deserialize)]
struct AcoustIDRecording {
    /// musicbrainz recording id
    id: String,
}

#[derive(Clone)]
pub struct AcoustID {
    api_key: String,
    client: reqwest::Client,
    limiter: Arc<leaky_bucket::RateLimiter>,
}

impl AcoustID {
    const LOOKUP_URL: &'static str = "https://api.acoustid.org/v2/lookup";

    pub fn new(api_key: impl AsRef<str>) -> crate::Result<Self> {
        let client = reqwest::Client::builder()
            .user_agent("music-organizer/0.1.0")
            .build()?;
        let limiter = leaky_bucket::RateLimiter::builder()
            .max(3)
            .initial(3)
            .refill(1)
            .interval(Duration::from_millis(350))
            .build();

        Ok(Self {
            api_key: api_key.as_ref().to_string(),
            client,
            limiter: Arc::new(limiter),
        })
    }

    /// Tries to find a musicbrainz recording id for the given fingerprint.
    pub async fn lookup(
        &self,
        fingerprint: impl AsRef<str>,
        duration: f64,
    ) -> crate::Result<Option<String>> {
        self.limiter.acquire(1).await;

        let params = [
            ("client", self.api_key.to_string()),
            ("meta", "recordingids".to_string()),
            ("duration", duration.round().to_string()),
            ("fingerprint", fingerprint.as_ref().to_string()),
        ];

        let response: AcoustIDResponse = self
            .client
            .get(Self::LOOKUP_URL)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        Ok(response
            .results
            .into_iter()
            .max_by(|a, b| a.score.partial_cmp(&b.score).unwrap())
            .and_then(|res| res.recordings.first().map(|r| r.id.clone())))
    }
}
