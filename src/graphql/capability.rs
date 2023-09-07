use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest;
use strum::EnumString;
use chrono::NaiveDateTime;

type UUID = String;

#[derive(Debug, PartialEq, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
/// Enums for Capability
pub enum CapabilityLevel {
    Desired,
    Novice,
    Experienced,
    Expert,
    Specialist
}

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/capabilities/capability_by_name_and_level.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct CapabilityByNameAndLevel;

pub fn get_capability_by_name_and_level(name: String, level: CapabilityLevel, bearer: String) -> Result<capability_by_name_and_level::ResponseData, Box<dyn Error>> {

    let level_enum: CapabilityLevel;

    let request_body = CapabilityByNameAndLevel::build_query(capability_by_name_and_level::Variables {
        name,
        level,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/graphql")
        .header("Bearer", bearer)
        .json(&request_body)
        .send()?;

    let response_body: Response<capability_by_name_and_level::ResponseData> = res.json()?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response = response_body.data
        .expect("missing response data");

    // serve HTML page with response_body
    Ok(response)
}