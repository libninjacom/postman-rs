use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateUserStateRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub user_id: String,
    pub schemas: Option<Vec<String>>,
    pub operations: Option<Vec<serde_json::Value>>,
}
impl<'a> UpdateUserStateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .patch(&format!("/scim/v2/Users/{user_id}", user_id = self.user_id));
        if let Some(ref unwrapped) = self.schemas {
            r = r.push_json(json!({ "schemas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.operations {
            r = r.push_json(json!({ "Operations" : unwrapped }));
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
    pub fn schemas(
        mut self,
        schemas: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .schemas = Some(
            schemas.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn operations(mut self, operations: Vec<serde_json::Value>) -> Self {
        self.operations = Some(operations);
        self
    }
}
