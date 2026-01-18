// [libs/domain/gamification/src/lib.rs]
/*!
 * APARATO: GAMIFICATION METRIC REGISTRY
 * RESPONSABILIDAD: Conversión de esfuerzo técnico en reputación soberana.
 */

use serde::{Deserialize, Serialize};
use typeshare::typeshare;

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorRank {
    pub level: u32,
    pub title: String,
    pub experience_points: u64,
    pub next_level_threshold: u64,
}

#[typeshare]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AchievementBadge {
    pub identifier: String,
    pub i18n_label_key: String,
    pub unlocked_at: String,
}
