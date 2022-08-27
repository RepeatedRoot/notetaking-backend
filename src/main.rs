use std::env;
use std::error::Error;
use bson::oid::ObjectId;
use tokio;
use chrono::{Utc, TimeZone};
use bson::{Bson};

mod client;
mod utils;

use client::{DatabaseMgr};
use utils::{User, Sex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the MongoDB connection string from an environment variable:
    /* let client_uri: String = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        
    println!("{}", client_uri);

    let client_manager: DatabaseMgr = DatabaseMgr::new(&client_uri).await?; */

    let test_client = User {
        id: None,
        surname: "Surname".to_owned(),
        given_names: "Firstname".to_owned(),
        middle_names: None,
        dob: Utc.ymd(1982, 04, 11).and_hms(0,0,0),
        sex: Sex::Male,
        address: "A street".to_owned(),
        postal_address: None,
        phone: 0412_212_212
    };

    /* let insert_result: ObjectId = client_manager.db_insert_document(&test_client).await?;

    let found_document = client_ma  nager.db_find_document(&insert_result).await?;

    println!("{:?}", found_document);

    client_manager.db_delete_document(&insert_result).await?; */

    let serialized = serde_json::to_string(&test_client).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: User = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let serialized_bson = bson::to_bson(&test_client)?;
    let serialized_document = serialized_bson.as_document().unwrap();
    println!("serialized bson = {}", serialized_bson);

    let deserialized_bson = bson::from_bson(serialized_document.into())?;
    println!("deserialized bson = {:?}", deserialized_bson);
    Ok(())
}
