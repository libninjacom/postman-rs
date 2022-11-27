use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateRelationsRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
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
            .http_client
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
