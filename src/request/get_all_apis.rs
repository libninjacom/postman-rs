use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAllApisRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace: Option<String>,
    pub since: Option<String>,
    pub until: Option<String>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub is_public: Option<bool>,
    pub name: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub sort: Option<String>,
    pub direction: Option<String>,
}
impl<'a> GetAllApisRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/apis");
        if let Some(ref unwrapped) = self.workspace {
            r = r.push_query("workspace", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.since {
            r = r.push_query("since", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.until {
            r = r.push_query("until", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_by {
            r = r.push_query("createdBy", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.updated_by {
            r = r.push_query("updatedBy", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.is_public {
            r = r.push_query("isPublic", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.summary {
            r = r.push_query("summary", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.description {
            r = r.push_query("description", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.direction {
            r = r.push_query("direction", &unwrapped.to_string());
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
    pub fn workspace(mut self, workspace: &str) -> Self {
        self.workspace = Some(workspace.to_owned());
        self
    }
    pub fn since(mut self, since: &str) -> Self {
        self.since = Some(since.to_owned());
        self
    }
    pub fn until(mut self, until: &str) -> Self {
        self.until = Some(until.to_owned());
        self
    }
    pub fn created_by(mut self, created_by: &str) -> Self {
        self.created_by = Some(created_by.to_owned());
        self
    }
    pub fn updated_by(mut self, updated_by: &str) -> Self {
        self.updated_by = Some(updated_by.to_owned());
        self
    }
    pub fn is_public(mut self, is_public: bool) -> Self {
        self.is_public = Some(is_public);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn summary(mut self, summary: &str) -> Self {
        self.summary = Some(summary.to_owned());
        self
    }
    pub fn description(mut self, description: &str) -> Self {
        self.description = Some(description.to_owned());
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
    pub fn direction(mut self, direction: &str) -> Self {
        self.direction = Some(direction.to_owned());
        self
    }
}
