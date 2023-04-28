use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;

use self::person_by_name::ResponseData;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/person_by_name.graphql",
    response_derives = "Debug"
)]
pub struct PersonByName;

pub fn get_people_by_name(name: String) -> Result<(), Box<dyn Error>> {

    let request_body = PersonByName::build_query(person_by_name::Variables {
        name,
    });

    let client = reqwest::blocking::Client::new();
    let mut res = client.post("http://127.0.0.1:8080/graphql").json(&request_body).send()?;

    let response_body: Response<person_by_name::ResponseData> = res.json()?;

    if let Some(errors) = response_body.errors {
        println!("there are errors:");

        for error in &errors {
            println!("{:?}", error);
        }
    };

    let response_data: person_by_name::ResponseData = response_body.data.expect("missing response data");

    println!("{:?}", &response_data);

    // serve HTML page with response_body
    Ok(())
}