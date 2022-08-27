#[path = "../src/client.rs"]
mod client;
#[path = "../src/utils.rs"]
mod utils;

use std::error::Error;

use chrono::{Utc, TimeZone};
use utils::{Sex, User};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_serialize_user() -> Result<(), Box<dyn Error>> {
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

    let serialized_bson = bson::to_bson(&test_client)?;
    let serialized_document = serialized_bson.as_document().unwrap();

    Ok(())
  }
}