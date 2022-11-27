use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAnApiVersionRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub version: Option<serde_json::Value>,
}
impl<'a> UpdateAnApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .put(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}", api_id = self.api_id,
                    api_version_id = self.api_version_id
                ),
            );
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
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
    pub fn version(mut self, version: serde_json::Value) -> Self {
        self.version = Some(version);
        self
    }
}
