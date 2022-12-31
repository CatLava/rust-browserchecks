
use std::{io::{Error, Write, Read}, fs};
use serde_derive::{Deserialize, Serialize};
use actix_web::{HttpResponse};
use std::fs::File;
use super::utils::show_results;

pub async fn show() -> HttpResponse {
    let data = read_data_store().unwrap_or("no data".to_string());
    format!("show view");
    //data;
    HttpResponse::Ok().json(&data)
}

fn read_data_store() -> Result<String, serde_json::Error> {
    println!("hi");
    //let fil = "./bfp.json".to_string();
    //let data = fs::read_to_string(fil);
    let data2 = show_results().unwrap();
    let json = serde_json::to_string_pretty(&data2);
    json
}