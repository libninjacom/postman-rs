#![allow(unused_imports)]
use postman2::PostmanClient;
use postman2::model::*;
use postman2::request::SyncRelationsWithSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = SyncRelationsWithSchemaRequired {
        api_version_id: "your api version id",
        relation_type: "your relation type",
        api_id: "your api id",
        entity_id: "your entity id",
    };
    let response = client.sync_relations_with_schema(args).send().await.unwrap();
    println!("{:#?}", response);
}
