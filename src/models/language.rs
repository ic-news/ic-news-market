use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Language {
    pub language: String,
    pub language_code: String,
    pub country_code: String,
    pub enabled: bool,
    pub updated_at: u64,
}