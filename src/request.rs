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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateApiRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub api: Option<serde_json::Value>,
}
impl<'a> CreateApiRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/apis");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.api {
            r = r.push_json(json!({ "api" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn api(mut self, api: serde_json::Value) -> Self {
        self.api = Some(api);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleApiRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
}
impl<'a> SingleApiRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/apis/{api_id}", api_id = self.api_id));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAnApiRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api: Option<serde_json::Value>,
}
impl<'a> UpdateAnApiRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/apis/{api_id}", api_id = self.api_id));
        if let Some(ref unwrapped) = self.api {
            r = r.push_json(json!({ "api" : unwrapped }));
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
    pub fn api(mut self, api: serde_json::Value) -> Self {
        self.api = Some(api);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAnApiRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
}
impl<'a> DeleteAnApiRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/apis/{api_id}", api_id = self.api_id));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAllApiVersionsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
}
impl<'a> GetAllApiVersionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/apis/{api_id}/versions", api_id = self.api_id));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateApiVersionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub version: Option<serde_json::Value>,
}
impl<'a> CreateApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/apis/{api_id}/versions", api_id = self.api_id));
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
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
    pub fn version(mut self, version: serde_json::Value) -> Self {
        self.version = Some(version);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetAnApiVersionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetAnApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}", api_id = self.api_id,
                    api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateAnApiVersionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub version: Option<serde_json::Value>,
}
impl<'a> UpdateAnApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}", api_id = self.api_id,
                    api_version_id = self.api_version_id
                ),
            );
        if let Some(ref unwrapped) = self.version {
            r = r.push_json(json!({ "version" : unwrapped }));
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
    pub fn version(mut self, version: serde_json::Value) -> Self {
        self.version = Some(version);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteAnApiVersionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> DeleteAnApiVersionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}", api_id = self.api_id,
                    api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContractTestRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetContractTestRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/contracttest", api_id =
                    self.api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetDocumentationRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetDocumentationRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/documentation", api_id =
                    self.api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetEnvironmentRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetEnvironmentRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/environment", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetIntegrationTestRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetIntegrationTestRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/integrationtest", api_id =
                    self.api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMockServerRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetMockServerRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/mock", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetMonitorRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetMonitorRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/monitor", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetLinkedRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetLinkedRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/relations", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub documentation: Option<Vec<String>>,
    pub environment: Option<Vec<String>>,
    pub mock: Option<Vec<String>>,
    pub monitor: Option<Vec<String>>,
    pub test: Option<Vec<String>>,
    pub contracttest: Option<Vec<String>>,
    pub testsuite: Option<Vec<String>>,
}
impl<'a> CreateRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/relations", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
        if let Some(ref unwrapped) = self.documentation {
            r = r.push_json(json!({ "documentation" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.environment {
            r = r.push_json(json!({ "environment" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.mock {
            r = r.push_json(json!({ "mock" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.monitor {
            r = r.push_json(json!({ "monitor" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.test {
            r = r.push_json(json!({ "test" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.contracttest {
            r = r.push_json(json!({ "contracttest" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.testsuite {
            r = r.push_json(json!({ "testsuite" : unwrapped }));
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
    pub fn documentation(
        mut self,
        documentation: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .documentation = Some(
            documentation.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn environment(
        mut self,
        environment: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .environment = Some(
            environment.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn mock(mut self, mock: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.mock = Some(mock.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn monitor(
        mut self,
        monitor: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .monitor = Some(
            monitor.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn test(mut self, test: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.test = Some(test.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn contracttest(
        mut self,
        contracttest: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .contracttest = Some(
            contracttest.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn testsuite(
        mut self,
        testsuite: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .testsuite = Some(
            testsuite.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema: Option<serde_json::Value>,
}
impl<'a> CreateSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
        if let Some(ref unwrapped) = self.schema {
            r = r.push_json(json!({ "schema" : unwrapped }));
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
    pub fn schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema_id: String,
}
impl<'a> GetSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas/{schema_id}",
                    api_id = self.api_id, api_version_id = self.api_version_id, schema_id
                    = self.schema_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema_id: String,
    pub schema: Option<serde_json::Value>,
}
impl<'a> UpdateSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas/{schema_id}",
                    api_id = self.api_id, api_version_id = self.api_version_id, schema_id
                    = self.schema_id
                ),
            );
        if let Some(ref unwrapped) = self.schema {
            r = r.push_json(json!({ "schema" : unwrapped }));
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
    pub fn schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCollectionFromSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub schema_id: String,
    pub workspace_id: Option<String>,
    pub name: String,
    pub relations: Vec<serde_json::Value>,
}
impl<'a> CreateCollectionFromSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/schemas/{schema_id}/collections",
                    api_id = self.api_id, api_version_id = self.api_version_id, schema_id
                    = self.schema_id
                ),
            );
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "relations" : self.relations }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
}
pub struct CreateCollectionFromSchemaRequired<'a> {
    pub api_id: &'a str,
    pub api_version_id: &'a str,
    pub schema_id: &'a str,
    pub name: &'a str,
    pub relations: Vec<serde_json::Value>,
}
impl<'a> CreateCollectionFromSchemaRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTestRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetTestRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/test", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetTestSuiteRelationsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
}
impl<'a> GetTestSuiteRelationsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/testsuite", api_id = self
                    .api_id, api_version_id = self.api_version_id
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SyncRelationsWithSchemaRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub api_id: String,
    pub api_version_id: String,
    pub relation_type: String,
    pub entity_id: String,
}
impl<'a> SyncRelationsWithSchemaRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/apis/{api_id}/versions/{api_version_id}/{relation_type}/{entity_id}/syncWithSchema",
                    api_id = self.api_id, api_version_id = self.api_version_id,
                    relation_type = self.relation_type, entity_id = self.entity_id
                ),
            );
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
}
pub struct SyncRelationsWithSchemaRequired<'a> {
    pub api_id: &'a str,
    pub api_version_id: &'a str,
    pub relation_type: &'a str,
    pub entity_id: &'a str,
}
impl<'a> SyncRelationsWithSchemaRequired<'a> {}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AllCollectionsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
}
impl<'a> AllCollectionsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/collections");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCollectionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub collection: Option<serde_json::Value>,
}
impl<'a> CreateCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/collections");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.collection {
            r = r.push_json(json!({ "collection" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn collection(mut self, collection: serde_json::Value) -> Self {
        self.collection = Some(collection);
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct MergeAForkRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub destination: Option<String>,
    pub source: Option<String>,
    pub strategy: Option<String>,
}
impl<'a> MergeAForkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/collections/merge");
        if let Some(ref unwrapped) = self.destination {
            r = r.push_json(json!({ "destination" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.source {
            r = r.push_json(json!({ "source" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.strategy {
            r = r.push_json(json!({ "strategy" : unwrapped }));
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
    pub fn destination(mut self, destination: &str) -> Self {
        self.destination = Some(destination.to_owned());
        self
    }
    pub fn source(mut self, source: &str) -> Self {
        self.source = Some(source.to_owned());
        self
    }
    pub fn strategy(mut self, strategy: &str) -> Self {
        self.strategy = Some(strategy.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleCollectionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub collection_uid: String,
}
impl<'a> SingleCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/collections/{collection_uid}", collection_uid = self.collection_uid
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateCollectionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub collection_uid: String,
    pub collection: Option<serde_json::Value>,
}
impl<'a> UpdateCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/collections/{collection_uid}", collection_uid = self.collection_uid
                ),
            );
        if let Some(ref unwrapped) = self.collection {
            r = r.push_json(json!({ "collection" : unwrapped }));
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
    pub fn collection(mut self, collection: serde_json::Value) -> Self {
        self.collection = Some(collection);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteCollectionRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub collection_uid: String,
}
impl<'a> DeleteCollectionRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/collections/{collection_uid}", collection_uid = self.collection_uid
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AllEnvironmentsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
}
impl<'a> AllEnvironmentsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/environments");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateEnvironmentRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub environment: Option<serde_json::Value>,
}
impl<'a> CreateEnvironmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/environments");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.environment {
            r = r.push_json(json!({ "environment" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn environment(mut self, environment: serde_json::Value) -> Self {
        self.environment = Some(environment);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleEnvironmentRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub environment_uid: String,
}
impl<'a> SingleEnvironmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/environments/{environment_uid}", environment_uid = self
                    .environment_uid
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateEnvironmentRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub environment_uid: String,
    pub environment: Option<serde_json::Value>,
}
impl<'a> UpdateEnvironmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(
                &format!(
                    "/environments/{environment_uid}", environment_uid = self
                    .environment_uid
                ),
            );
        if let Some(ref unwrapped) = self.environment {
            r = r.push_json(json!({ "environment" : unwrapped }));
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
    pub fn environment(mut self, environment: serde_json::Value) -> Self {
        self.environment = Some(environment);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteEnvironmentRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub environment_uid: String,
}
impl<'a> DeleteEnvironmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/environments/{environment_uid}", environment_uid = self
                    .environment_uid
                ),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ImportExportedDataRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> ImportExportedDataRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/import/exported");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ImportExternalApiSpecificationRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub body: serde_json::Value,
}
impl<'a> ImportExternalApiSpecificationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/import/openapi");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        r = r.push_json(json!({ "body" : self.body }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ApiKeyOwnerRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> ApiKeyOwnerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/me");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AllMocksRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> AllMocksRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/mocks");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub mock: Option<serde_json::Value>,
}
impl<'a> CreateMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/mocks");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.mock {
            r = r.push_json(json!({ "mock" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn mock(mut self, mock: serde_json::Value) -> Self {
        self.mock = Some(mock);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub mock_uid: String,
}
impl<'a> SingleMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/mocks/{mock_uid}", mock_uid = self.mock_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub mock_uid: String,
    pub mock: Option<serde_json::Value>,
}
impl<'a> UpdateMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/mocks/{mock_uid}", mock_uid = self.mock_uid));
        if let Some(ref unwrapped) = self.mock {
            r = r.push_json(json!({ "mock" : unwrapped }));
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
    pub fn mock(mut self, mock: serde_json::Value) -> Self {
        self.mock = Some(mock);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub mock_uid: String,
}
impl<'a> DeleteMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/mocks/{mock_uid}", mock_uid = self.mock_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PublishMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub mock_uid: String,
}
impl<'a> PublishMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(&format!("/mocks/{mock_uid}/publish", mock_uid = self.mock_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnpublishMockRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub mock_uid: String,
}
impl<'a> UnpublishMockRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/mocks/{mock_uid}/unpublish", mock_uid = self.mock_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AllMonitorsRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> AllMonitorsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/monitors");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateMonitorRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub monitor: Option<serde_json::Value>,
}
impl<'a> CreateMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/monitors");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.monitor {
            r = r.push_json(json!({ "monitor" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn monitor(mut self, monitor: serde_json::Value) -> Self {
        self.monitor = Some(monitor);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleMonitorRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub monitor_uid: String,
}
impl<'a> SingleMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/monitors/{monitor_uid}", monitor_uid = self.monitor_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMonitorRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub monitor_uid: String,
    pub monitor: Option<serde_json::Value>,
}
impl<'a> UpdateMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .put(&format!("/monitors/{monitor_uid}", monitor_uid = self.monitor_uid));
        if let Some(ref unwrapped) = self.monitor {
            r = r.push_json(json!({ "monitor" : unwrapped }));
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
    pub fn monitor(mut self, monitor: serde_json::Value) -> Self {
        self.monitor = Some(monitor);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteMonitorRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub monitor_uid: String,
}
impl<'a> DeleteMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(&format!("/monitors/{monitor_uid}", monitor_uid = self.monitor_uid));
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct RunAMonitorRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub monitor_uid: String,
}
impl<'a> RunAMonitorRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .post(
                &format!("/monitors/{monitor_uid}/run", monitor_uid = self.monitor_uid),
            );
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetResourceTypesRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> GetResourceTypesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Vec<serde_json::Value>> {
        let mut r = self.client.client.get("/scim/v2/ResourceTypes");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ServiceProviderConfigRequest<'a> {
    pub(crate) client: &'a PostmanClient,
}
impl<'a> ServiceProviderConfigRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/scim/v2/ServiceProviderConfig");
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
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct FetchAllUserResourceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub start_index: Option<f64>,
    pub count: Option<f64>,
    pub filter: Option<String>,
}
impl<'a> FetchAllUserResourceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/scim/v2/Users");
        if let Some(ref unwrapped) = self.start_index {
            r = r.push_query("startIndex", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.count {
            r = r.push_query("count", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.filter {
            r = r.push_query("filter", &unwrapped.to_string());
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
    pub fn start_index(mut self, start_index: f64) -> Self {
        self.start_index = Some(start_index);
        self
    }
    pub fn count(mut self, count: f64) -> Self {
        self.count = Some(count);
        self
    }
    pub fn filter(mut self, filter: &str) -> Self {
        self.filter = Some(filter.to_owned());
        self
    }
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct FetchUserResourceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub user_id: String,
}
impl<'a> FetchUserResourceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(&format!("/scim/v2/Users/{user_id}", user_id = self.user_id));
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
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateUserStateRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub user_id: String,
    pub schemas: Option<Vec<String>>,
    pub operations: Option<Vec<serde_json::Value>>,
}
impl<'a> UpdateUserStateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .patch(&format!("/scim/v2/Users/{user_id}", user_id = self.user_id));
        if let Some(ref unwrapped) = self.schemas {
            r = r.push_json(json!({ "schemas" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.operations {
            r = r.push_json(json!({ "Operations" : unwrapped }));
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
    pub fn operations(mut self, operations: Vec<serde_json::Value>) -> Self {
        self.operations = Some(operations);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SchemaSecurityValidationRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub schema: Option<serde_json::Value>,
}
impl<'a> SchemaSecurityValidationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/security/api-validation");
        if let Some(ref unwrapped) = self.schema {
            r = r.push_json(json!({ "schema" : unwrapped }));
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
    pub fn schema(mut self, schema: serde_json::Value) -> Self {
        self.schema = Some(schema);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateWebhookRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: Option<String>,
    pub webhook: Option<serde_json::Value>,
}
impl<'a> CreateWebhookRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/webhooks");
        if let Some(ref unwrapped) = self.workspace_id {
            r = r.push_query("workspaceId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.webhook {
            r = r.push_json(json!({ "webhook" : unwrapped }));
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
    pub fn workspace_id(mut self, workspace_id: &str) -> Self {
        self.workspace_id = Some(workspace_id.to_owned());
        self
    }
    pub fn webhook(mut self, webhook: serde_json::Value) -> Self {
        self.webhook = Some(webhook);
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct AllWorkspacesRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub type_: Option<String>,
}
impl<'a> AllWorkspacesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.get("/workspaces");
        if let Some(ref unwrapped) = self.type_ {
            r = r.push_query("type", &unwrapped.to_string());
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
    pub fn type_(mut self, type_: &str) -> Self {
        self.type_ = Some(type_.to_owned());
        self
    }
}
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateWorkspaceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace: Option<serde_json::Value>,
}
impl<'a> CreateWorkspaceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.client.client.post("/workspaces");
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct SingleWorkspaceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: String,
}
impl<'a> SingleWorkspaceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .get(
                &format!("/workspaces/{workspace_id}", workspace_id = self.workspace_id),
            );
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
}
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
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteWorkspaceRequest<'a> {
    pub(crate) client: &'a PostmanClient,
    pub workspace_id: String,
}
impl<'a> DeleteWorkspaceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self
            .client
            .client
            .delete(
                &format!("/workspaces/{workspace_id}", workspace_id = self.workspace_id),
            );
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
}
