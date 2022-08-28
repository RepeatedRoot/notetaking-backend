#[path = "../src/client.rs"]
mod client;
#[path = "../src/utils.rs"]
mod utils;

use utils::{Sex, User, Workplace, Clinician, Qualification};

use std::error::Error;

use chrono::{Utc, TimeZone};
use bson::{Bson, document::Document};
use mongodb::bson::oid::ObjectId;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn serialize_user() -> Result<(), Box<dyn Error>> {
    let test_client: User = User {
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

    let serialized_bson: Bson = bson::to_bson(&test_client)?;
    let serialized_document: &Document = serialized_bson.as_document().unwrap();

    let deserialized_document: User = bson::from_bson(serialized_document.into())?;

    assert_eq!(test_client, deserialized_document);

    Ok(())
  }

  #[test]
  fn serialize_workplace() -> Result<(), Box<dyn Error>> {
    let test_workplace: Workplace = Workplace { 
      id: None,
      name: "Workplace".to_owned(),
      address: "A street".to_owned(),
      phone: 8339_0123
    };

    let serialized_bson: Bson = bson::to_bson(&test_workplace)?;
    let serialized_document: &Document = serialized_bson.as_document().unwrap();

    let deserialized_document: Workplace = bson::from_bson(serialized_document.into())?;

    assert_eq!(test_workplace, deserialized_document);

    Ok(())
  }

  #[test]
  fn serialize_clinician() -> Result<(), Box<dyn Error>> {
    let test_clinician: Clinician = Clinician {
      id: None,
      first_name: "Firstname".to_owned(),
      last_name: "Lastname".to_owned(),
      phone: 0410_221_175,
      workplace: ObjectId::new(),
      qualification: Qualification::RegisteredNurse
    };
    
    let serialized_bson: Bson = bson::to_bson(&test_clinician)?;
    let serialized_document: &Document = serialized_bson.as_document().unwrap();

    let deserialized_document: Clinician = bson::from_bson(serialized_document.into())?;

    assert_eq!(test_clinician, deserialized_document);

    Ok(())
  }
}