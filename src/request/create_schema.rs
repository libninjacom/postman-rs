use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema: Option<serde_json::Value>,
}
impl<'a> CreateSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
        if let Some(ref unwrapped) = self.schema {
            r = r.push_json(json!({ "schema" : unwrapped }));
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
    pub fn schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }
}
