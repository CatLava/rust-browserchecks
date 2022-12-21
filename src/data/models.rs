use chrono::{NaiveDateTime, Utc};

// Look into timestamp and Id next
pub struct NewBfpData {
    pub bfp_hash: String,
    pub user_agent: String,
}

pub struct BfpData {
    pub id: i32,
    pub bfp_hash: String,
    pub user_agent: String,
}