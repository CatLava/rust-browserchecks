use serde_json;
use serde::{Deserialize, Serialize};
use actix_web::{web, HttpRequest, Responder};
use std::{io::{Error, Write}, fs, vec};
use std::hash::{Hasher, Hash};
use serde_derive::{Deserialize, Serialize};
use md5::{Md5};
use md5::Digest;
use crate::data::models::{NewBfpData, BfpData};
use crate::data::query::establish_connection;
use crate::schema::bfp;
use diesel::RunQueryDsl;
use diesel::prelude::*;


pub fn show_results() -> Result<Vec<BfpData>, String> {

    use crate::schema::bfp::dsl::bfp;

    let connection = &mut establish_connection();
    let results: Vec<BfpData>  = bfp
        .limit(25)
        .load::<BfpData>(connection)
        .expect("Error loading posts");
    println!("restul;s totaly{:?}", results);

    println!("Displaying {} posts", results.len());
    for post in &results {
        println!("{}", post.bfp_hash);
        println!("-----------\n");
        println!("{}", post.user_agent);
    }
    Ok(results)
}