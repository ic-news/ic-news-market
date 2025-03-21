use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Channel {
    pub name: String,
    pub platform: String,
    pub enabled: bool,
    pub updated_at: u64,
}