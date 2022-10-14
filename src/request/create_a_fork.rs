use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateAForkRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace: String,
    pub collection_uid: String,
    pub label: Option<String>,
}
impl<'a> CreateAForkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/collections/fork/{collection_uid}", collection_uid = self
                    .collection_uid
                ),
            );
        r = r.push_query("workspace", &self.workspace.to_string());
        if let Some(ref unwrapped) = self.label {
            r = r.push_json(json!({ "label" : unwrapped }));
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
    pub fn label(mut self, label: &str) -> Self {
        self.label = Some(label.to_owned());
        self
    }
}
