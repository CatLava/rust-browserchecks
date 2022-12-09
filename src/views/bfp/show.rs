
use std::{io::{Error, Write, Read}, fs};
use serde_derive::{Deserialize, Serialize};
use std::fs::File;

pub async fn show() -> String {
    let data = read_data_store().unwrap_or("no data".to_string());
    format!("show view");
    data
}

fn read_data_store() -> Result<String, Error> {
    println!("hi");
    let fil = "./bfp.json".to_string();
    let data = fs::read_to_string(fil);
    data
}