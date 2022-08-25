use mongodb::{bson, Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;
use chrono::{Utc, TimeZone};

mod modules;
mod validation;
//mod clinicion;

use modules::{User, UserMgr, Sex};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    
   println!("{}", client_uri);
   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;
   
   let user_manager = UserMgr::new(&client_uri).await?;

   let mut test_user: User = User {
       id: None,
       surname: Some("Smith".to_owned()),
       given_names: Some("Jones".to_owned()),
       middle_names: None,
       dob: Utc.ymd(1980, 3, 2).and_hms(0, 0, 0),
       sex: Some(Sex::Male),
       address: Some("This is a house at a street".to_owned()),
       postal_address: None,
       phone: Some(8272_0444),
   };

   println!("{:?}", test_user);

   let serialized_user = bson::to_bson(&test_user)?;
   let document = serialized_user.as_document().unwrap();

   let inserted_result = user_manager.db_insert_client(&mut test_user).await?;
   println!("Inserted _id: {:?}", inserted_result);
 
   Ok(())
}
