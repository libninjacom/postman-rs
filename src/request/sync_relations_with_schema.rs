use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SyncRelationsWithSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub relation_type: String,
    pub entity_id: String,
}
impl<'a> SyncRelationsWithSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/{relation_type}/{entity_id}/syncWithSchema",
                    api_id = self.api_id, api_version_id = self.api_version_id,
                    relation_type = self.relation_type, entity_id = self.entity_id
                ),
            );
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
}
pub struct SyncRelationsWithSchemaRequired<'a> {
    pub api_id: &'a str,
    pub api_version_id: &'a str,
    pub relation_type: &'a str,
    pub entity_id: &'a str,
}
impl<'a> SyncRelationsWithSchemaRequired<'a> {}
