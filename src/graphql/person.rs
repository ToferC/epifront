use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest;
use uuid::Uuid;

type UUID = String;
#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/person_by_name.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct PersonByName;

pub fn get_people_by_name(name: String, bearer: String) -> Result<person_by_name::ResponseData, Box<dyn Error>> {

    let request_body = PersonByName::build_query(person_by_name::Variables {
        name,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/graphql")
        .header("Bearer", bearer)
        .json(&request_body)
        .send()?;

    let response_body: Response<person_by_name::ResponseData> = res.json()?;

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

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/person_by_id.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct PersonById;

pub fn get_person_by_id(id: UUID, bearer: String) -> Result<person_by_id::ResponseData, Box<dyn Error>> {

    let request_body = PersonById::build_query(person_by_id::Variables {
        id,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/graphql")
        .header("Bearer", bearer)
        .json(&request_body)
        .send()?;

    let response_body: Response<person_by_id::ResponseData> = res.json()?;

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