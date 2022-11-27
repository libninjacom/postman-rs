use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCollectionFromSchemaRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema_id: String,
    pub workspace_id: Option<String>,
    pub name: String,
    pub relations: Vec<serde_json::Value>,
}
impl<'a> CreateCollectionFromSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas/{schema_id}/collections",
                    api_id = self.api_id, api_version_id = self.api_version_id, schema_id
                    = self.schema_id
                ),
            );
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "relations" : self.relations }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
}
pub struct CreateCollectionFromSchemaRequired<'a> {
    pub api_id: &'a str,
    pub api_version_id: &'a str,
    pub schema_id: &'a str,
    pub name: &'a str,
    pub relations: Vec<serde_json::Value>,
}
impl<'a> CreateCollectionFromSchemaRequired<'a> {}
