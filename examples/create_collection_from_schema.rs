use postman2::PostmanClient;
use postman2::model::*;
use postman2::request::CreateCollectionFromSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = CreateCollectionFromSchemaRequired {
        api_version_id: "your api version id",
        api_id: "your api id",
        schema_id: "your schema id",
        name: "your name",
        relations: vec![::serde_json::json!({})],
    };
    let response = client
        .create_collection_from_schema(args)
        .workspace_id("your workspace id")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
