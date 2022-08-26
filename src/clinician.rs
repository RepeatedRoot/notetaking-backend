use serde::{Deserialize, Serialize};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    {Client, Collection},
};
use chrono::{TimeZone, Utc};
use std::error::Error;

/* The possible qualifications of a clinicion */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Qualification {
    //Qualifications
}

/* The workplace struct */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Workplace {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub address: String,
    pub phone: u32,
}

/* Details about a clinician */
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clinician {
    id: Option<ObjectId>,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<u32>,
    workplace: Option<ObjectId>,
    qualification: Option<Qualification>
}
