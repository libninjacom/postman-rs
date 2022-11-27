use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMockRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub mock_uid: String,
    pub mock: Option<serde_json::Value>,
}
impl<'a> UpdateMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .put(&format!("/mocks/{mock_uid}", mock_uid = self.mock_uid));
        if let Some(ref unwrapped) = self.mock {
            r = r.push_json(json!({ "mock" : unwrapped }));
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
    pub fn mock(mut self, mock: serde_json::Value) -> Self {
        self.mock = Some(mock);
        self
    }
}
