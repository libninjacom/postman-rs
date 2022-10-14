use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateUserRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub schemas: Option<Vec<String>>,
    pub user_name: Option<String>,
    pub active: Option<bool>,
    pub external_id: Option<String>,
    pub groups: Option<Vec<String>>,
    pub locale: Option<String>,
    pub name: Option<serde_json::Value>,
}
impl<'a> CreateUserRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/scim/v2/Users");
        if let Some(ref unwrapped) = self.schemas {
            r = r.push_json(json!({ "schemas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.user_name {
            r = r.push_json(json!({ "userName" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.active {
            r = r.push_json(json!({ "active" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.external_id {
            r = r.push_json(json!({ "externalId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.groups {
            r = r.push_json(json!({ "groups" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.locale {
            r = r.push_json(json!({ "locale" : unwrapped }));
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
    pub fn user_name(mut self, user_name: &str) -> Self {
        self.user_name = Some(user_name.to_owned());
        self
    }
    pub fn active(mut self, active: bool) -> Self {
        self.active = Some(active);
        self
    }
    pub fn external_id(mut self, external_id: &str) -> Self {
        self.external_id = Some(external_id.to_owned());
        self
    }
    pub fn groups(mut self, groups: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.groups = Some(groups.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn locale(mut self, locale: &str) -> Self {
        self.locale = Some(locale.to_owned());
        self
    }
    pub fn name(mut self, name: serde_json::Value) -> Self {
        self.name = Some(name);
        self
    }
}
