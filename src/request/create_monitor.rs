use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateMonitorRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub monitor: Option<serde_json::Value>,
}
impl<'a> CreateMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.http_client.client.post("/monitors");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.monitor {
            r = r.push_json(json!({ "monitor" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn monitor(mut self, monitor: serde_json::Value) -> Self {
        self.monitor = Some(monitor);
        self
    }
}
