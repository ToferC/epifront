use graphql_client::{GraphQLQuery, Response};
use serde::{Serialize, Deserialize};
use std::error::Error;
use reqwest;
use uuid::Uuid;

type UUID = String;

#[derive(GraphQLQuery, Serialize, Deserialize)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/role_by_id.graphql",
    response_derives = "Debug, Serialize, PartialEq"
)]
pub struct RoleById;

pub fn get_role_by_id(id: UUID, bearer: String) -> Result<role_by_id::ResponseData, Box<dyn Error>> {

    let request_body = RoleById::build_query(role_by_id::Variables {
        id,
    });

    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://127.0.0.1:8080/graphql")
        .header("Bearer", bearer)
        .json(&request_body)
        .send()?;

    let response_body: Response<role_by_id::ResponseData> = res.json()?;

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