use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMonitorRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub monitor_uid: String,
    pub monitor: Option<serde_json::Value>,
}
impl<'a> UpdateMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/monitors/{monitor_uid}", monitor_uid = self.monitor_uid));
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
    pub fn monitor(mut self, monitor: serde_json::Value) -> Self {
        self.monitor = Some(monitor);
        self
    }
}
