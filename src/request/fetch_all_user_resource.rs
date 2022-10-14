use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
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
