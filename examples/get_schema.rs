use postman_api::PostmanClient;
use postman_api::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let schema_id = "your schema id";
    let response = client
        .get_schema(api_id, api_version_id, schema_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
