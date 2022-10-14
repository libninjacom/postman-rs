use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateUserInformationRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub user_id: String,
    pub schemas: Option<Vec<String>>,
    pub name: Option<serde_json::Value>,
}
impl<'a> UpdateUserInformationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/scim/v2/Users/{user_id}", user_id = self.user_id));
        if let Some(ref unwrapped) = self.schemas {
            r = r.push_json(json!({ "schemas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
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
    pub fn name(mut self, name: serde_json::Value) -> Self {
        self.name = Some(name);
        self
    }
}
