use chrono::{NaiveDateTime, Utc};
use serde_derive::Serialize;
use crate::schema::bfp;
use diesel::prelude::*;

// Look into timestamp and Id next
#[derive(Insertable)]
#[diesel(table_name=bfp)]
pub struct NewBfpData<'a> {
    pub bfp_hash: &'a str,
    pub user_agent: &'a str,
}

// impl NewBfpData<'a> {
//     pub fn new(bfp_hash: &str, user_agent: &str) -> NewBfpData<'a> {
//         return NewBfpData{ bfp_hash, user_agent}
//     }
// }

#[derive(Queryable, Identifiable, Debug, Serialize)]
#[diesel(table_name = bfp)]
pub struct BfpData {
    pub id: i32,
    pub bfp_hash: String,
    pub user_agent: String,
}