use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateEnvironmentRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub environment_uid: String,
    pub environment: Option<serde_json::Value>,
}
impl<'a> UpdateEnvironmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/environments/{environment_uid}", environment_uid = self
                    .environment_uid
                ),
            );
        if let Some(ref unwrapped) = self.environment {
            r = r.push_json(json!({ "environment" : unwrapped }));
        }
        r = self.http_client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn environment(mut self, environment: serde_json::Value) -> Self {
        self.environment = Some(environment);
        self
    }
}
