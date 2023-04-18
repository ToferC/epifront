use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "queries/person_query.graphql",
    response_derives = "Debug"
)]
pub struct PersonQuery;

pub async fn get_people(variables: person_query::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = PersonQuery::build_query(variables);

    let client = reqwest::Client::new();
    let mut res = client.post("/graphql").json(&request_body).send().await?;

    let response_body: Response<person_query::ResponseData> = res.json().await?;
    Ok(())
}