// This will add to a db
// for testing this will be a json file
use serde_json;
use serde::{Deserialize, Serialize};
use actix_web::{HttpRequest, Responder};
use std::{io::{Error, Write}, fs};
use serde_derive::{Deserialize, Serialize};

pub async fn add_browser_info(req: HttpRequest)-> String {
    // TODO need to handle this unwrap poroperly
    let test_data = req.headers().get("user-agent").unwrap().to_str().unwrap();
    println!("http req: {:?}", test_data);

    let added = add_string_to_store(test_data);
    println!("added {:?}", added);
    format!("bfp data")

}


#[derive(Serialize, Deserialize, Debug)]
struct BfpAddition {
    index: i32,
    data: String
}

fn add_string_to_store(data: &str) -> Result<usize, Error> {
    // need to read file and get index count
    // just need rudimentary until db portion is added 
    let bfp_add = BfpAddition {
        index: 4,
        data: data.to_owned()
    };
    let fil = "./bfp.json".to_string();
    let mut openfile = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(fil)
        .unwrap();
    // ? returns a result if ok, or unpacks a result
    println!("bfp: {:?}", bfp_add);
    // std::fs::write(
    //     fil,
    //     serde_json::to_string_pretty(&bfp_add).unwrap(),
    // )
    let data_to_add: String = serde_json::to_string_pretty(&bfp_add).unwrap() + ",";
    openfile.write(data_to_add.as_bytes())

}