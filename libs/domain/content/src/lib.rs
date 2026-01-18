use serde::{Deserialize, Serialize};
use typeshare::typeshare;
use chrono::{DateTime, Utc};

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForensicArticle {
    pub id: String,
    pub title_key: String,
    pub body_markdown: String,
    pub published_at: DateTime<Utc>,
    pub category: String, // e.g., "Entropy_Deep_Dive"
}
