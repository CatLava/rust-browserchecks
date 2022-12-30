// This will add to a db
// for testing this will be a json file
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




// This must match exactly the input from FE
#[derive(Debug, Serialize, Deserialize, Hash )]
pub struct BFPFields {
    // For future references on structs, use the rename feature for these
    //#[serde(rename = "Cache-Control")]
    user_agent: String,
    screen_info: String,
    timezone: i32, // Timezone returns mintues off of GMT, so PST is 480
    session: bool,
    local_storage: bool,
    operating_system: String,
    cookie_enabled: bool,
    java_enabled: bool,
    lang: String,
    plugins: String,
}

impl BFPFields {
    fn calculate_hash(&self) -> String {
        // TODO, Implement interator for this
        // Iterator requires index to move on
        let fields: Vec<String> = vec![self.user_agent.to_owned(),
            self.screen_info.to_owned(),
            self.timezone.to_string(),
            self.session.to_string(),
            self.local_storage.to_string(),
            self.operating_system.to_owned(),
            self.cookie_enabled.to_string(),
            self.java_enabled.to_string(),
            self.lang.to_owned(),
            self.plugins.to_owned()
            ];
        let mut field_string = String::new();
        for item in fields.iter() {
            field_string.push_str(item)
        }
        let mut hasher = Md5::new();
        hasher.update(stringify!(fields).as_bytes());
        println!("datat a {:x}", md5::Md5::digest(stringify!(fields).as_bytes()));
        format!("{:x}", md5::Md5::digest(stringify!(fields).as_bytes()) )
    } 

}

#[derive(Debug, Serialize, Deserialize )]
pub struct BFPArray
    {
        value: BFPFields
      }

pub async fn add_browser_info(req: HttpRequest, data: web::Json<BFPArray>)-> String {
    // TODO need to handle this unwrap poroperly
   // println!("data {:?}", data.into_inner().value);
    println!("data {:?}", data.into_inner().value.calculate_hash());
    println!("req {:?}", req);
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
    use crate::schema::bfp;
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
    let mut connection = establish_connection();
    let new_bfp = NewBfpData {
        bfp_hash : &"hahs".to_string(),
        user_agent : &"agent".to_string(),
    };
    println!("making it to db call!!!");
    diesel::insert_into(bfp::table)
        .values(&new_bfp)
        .get_result::<BfpData>(&mut connection)
        .expect("Error saving new post");
       
        
    //     fil,
    //     serde_json::to_string_pretty(&bfp_add).unwrap(),
    // )
    let data_to_add: String = serde_json::to_string_pretty(&bfp_add).unwrap() + ",";
    openfile.write(data_to_add.as_bytes())

}