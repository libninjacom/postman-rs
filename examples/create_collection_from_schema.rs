#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
use postman::request::CreateCollectionFromSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = CreateCollectionFromSchemaRequired {
        name: "your name",
        relations: vec![::serde_json::json!({})],
        api_id: "your api id",
        api_version_id: "your api version id",
        schema_id: "your schema id",
    };
    let response = client
        .create_collection_from_schema(args)
        .workspace_id("your workspace id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
