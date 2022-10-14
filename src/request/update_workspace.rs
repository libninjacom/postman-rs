use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateWorkspaceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: String,
    pub workspace: Option<serde_json::Value>,
}
impl<'a> UpdateWorkspaceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!("/workspaces/{workspace_id}", workspace_id = self.workspace_id),
            );
        if let Some(ref unwrapped) = self.workspace {
            r = r.push_json(json!({ "workspace" : unwrapped }));
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
    pub fn workspace(mut self, workspace: serde_json::Value) -> Self {
        self.workspace = Some(workspace);
        self
    }
}
