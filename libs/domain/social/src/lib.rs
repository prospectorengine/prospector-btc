// [libs/domain/social/src/lib.rs]
use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialHandshake {
    pub operator_origin: String,
    pub message_content: String,
    pub timestamp: String,
}
