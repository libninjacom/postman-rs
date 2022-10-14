use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateApiVersionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub version: Option<serde_json::Value>,
}
impl<'a> CreateApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/apis/{api_id}/versions", api_id = self.api_id));
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
        }
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
    pub fn version(mut self, version: serde_json::Value) -> Self {
        self.version = Some(version);
        self
    }
}
