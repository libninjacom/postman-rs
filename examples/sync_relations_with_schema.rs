use postman_api::PostmanClient;
use postman_api::model::*;
use postman_api::request::SyncRelationsWithSchemaRequired;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let args = SyncRelationsWithSchemaRequired {
        api_version_id: "your api version id",
        api_id: "your api id",
        relation_type: "your relation type",
        entity_id: "your entity id",
    };
    let response = client.sync_relations_with_schema(args).send().await.unwrap();
    println!("{:#?}", response);
}
