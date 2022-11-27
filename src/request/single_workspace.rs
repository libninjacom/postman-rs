use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleWorkspaceRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub workspace_id: String,
}
impl<'a> SingleWorkspaceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!("/workspaces/{workspace_id}", workspace_id = self.workspace_id),
            );
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
}
