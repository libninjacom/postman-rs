use serde_json::json;
use crate::model::*;
use crate::PostmanClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct MergeAForkRequest<'a> {
    pub(crate) http_client: &'a PostmanClient,
    pub destination: Option<String>,
    pub source: Option<String>,
    pub strategy: Option<String>,
}
impl<'a> MergeAForkRequest<'a> {
    pub async fn send(self) -> anyhow::Result<serde_json::Value> {
        let mut r = self.http_client.client.post("/collections/merge");
        if let Some(ref unwrapped) = self.destination {
            r = r.push_json(json!({ "destination" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.source {
            r = r.push_json(json!({ "source" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.strategy {
            r = r.push_json(json!({ "strategy" : unwrapped }));
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
