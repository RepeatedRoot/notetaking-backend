use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, oid::ObjectId},
};
use chrono::{Utc};

/* The possible qualifications of a clinicion */
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum Qualification {
    MedicalOfficer,
    ClinicalNurse,
    RegisteredNurse,
    PhysioTherapist,
    FamilySupportOfficer,
    ClientServicesOfficer
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Male,
    Female,
    Other
}

/* Struct to store client information (can't use the name client because of MongoDB) */
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<bson::oid::ObjectId>,
    pub surname: String,
    pub given_names: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_names: Option<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub dob: chrono::DateTime<Utc>,
    pub sex: Sex,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<String>,
    pub phone: u32,
}

/* The workplace struct */
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Workplace {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub address: String,
    pub phone: u32,
}

/* Details about a clinician */
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Clinician {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub phone: u32,
    pub workplace: ObjectId,
    pub qualification: Qualification
}
