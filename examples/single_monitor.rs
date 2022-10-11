use postman2::PostmanClient;
use postman2::model::*;
#[tokio::main]
async fn main() {
    let client = PostmanClient::from_env();
    let monitor_uid = "your monitor uid";
    let response = client.single_monitor(monitor_uid).send().await.unwrap();
    println!("{:#?}", response);
}
