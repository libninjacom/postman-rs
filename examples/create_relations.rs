use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let api_id = "your api id";
    let api_version_id = "your api version id";
    let response = client
        .create_relations(api_id, api_version_id)
        .documentation(&["your documentation"])
        .environment(&["your environment"])
        .mock(&["your mock"])
        .monitor(&["your monitor"])
        .test(&["your test"])
        .contracttest(&["your contracttest"])
        .testsuite(&["your testsuite"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
