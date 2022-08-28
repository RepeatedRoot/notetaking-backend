use std::env;
use std::error::Error;

use bson::oid::ObjectId;
use tokio;
use chrono::{Utc, TimeZone};

mod client; //Database client
mod utils;  //Utilities (struct/enum definitions)

use client::{DatabaseMgr};
use utils::{User, Sex, Workplace};

const DB_NAME: &str = "CAFHS-notetaking";
const CLIENTS: &str = "clients";
const WORKPLACE: &str = "workplaces";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    let client_uri: String = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        
    println!("{}", client_uri);

    let client_manager: DatabaseMgr = DatabaseMgr::new(&client_uri, DB_NAME).await?;

    let test_workplace = Workplace {
        id: None,
        name: "CaFHS Morphett Vale".to_owned(),
        address: "211 Main S Rd, Morphett Vale SA 5162".to_owned(),
        phone: 1300_733_606
    };

    let insert_result = client_manager.db_insert_document(&test_workplace, WORKPLACE).await?;

    println!("{:?}", insert_result);

    Ok(())
}
