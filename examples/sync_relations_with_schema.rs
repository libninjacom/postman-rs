#![allow(unused_imports)]
use postman::PostmanClient;
use postman::model::*;
use postman::request::SyncRelationsWithSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = SyncRelationsWithSchemaRequired {
        api_id: "your api id",
        entity_id: "your entity id",
        relation_type: "your relation type",
        api_version_id: "your api version id",
    };
    let response = client.sync_relations_with_schema(args).send().await.unwrap();
    println!("{:#?}", response);
}
