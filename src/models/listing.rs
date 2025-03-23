use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, Deserialize, Serialize, CandidType)]
pub struct Platform {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub slug: String,
    pub token_address: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Listing {
    pub id: u32,
    pub symbol: String,
    pub name: String,
    pub slug: String,
    pub cmc_rank: u32,
    pub num_market_pairs: u32,
    pub circulating_supply: f64,
    pub total_supply: f64,
    pub max_supply: f64,
    pub infinite_supply: bool,
    pub date_added: String,
    pub tags: Vec<String>,
    pub platform: Option<Platform>,
    pub self_reported_circulating_supply: f64,
    pub self_reported_market_cap: f64,
    pub quote: Quote,
    pub last_updated: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Quote {
    pub price: f64,
    pub volume_24h: f64,
    pub volume_change_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub percent_change_30d: f64,
    pub percent_change_60d: f64,
    pub percent_change_90d: f64,
    pub market_cap: f64,
    pub market_cap_dominance: f64,
    pub fully_diluted_market_cap: f64,
    pub last_updated: String,
}