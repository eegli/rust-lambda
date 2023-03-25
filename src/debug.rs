use lambda_http::{
    aws_lambda_events::query_map::QueryMap,
    http::{HeaderMap, HeaderValue},
    Context,
};

use std::collections::HashMap;
use std::ops::Deref;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DebugResponse<T>
where
    T: serde::Serialize + Default,
{
    headers: QueryObject,
    query: QueryObject,
    method: String,
    path_raw: String,
    context: serde_json::Value,
    path_params: Vec<String>,
    payload: Option<T>,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct QueryObject(HashMap<String, String>);

impl Deref for QueryObject {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&HeaderMap<HeaderValue>> for QueryObject {
    fn from(hm: &HeaderMap<HeaderValue>) -> Self {
        let mut map = HashMap::new();
        for (k, v) in hm {
            let k = k.as_str().to_owned();
            let v = String::from_utf8_lossy(v.as_bytes()).into_owned();
            map.insert(k, v);
        }
        Self(map)
    }
}

impl From<&QueryMap> for QueryObject {
    fn from(qm: &QueryMap) -> Self {
        let mut map = HashMap::new();
        for (k, v) in qm.iter() {
            let k = k.to_owned();
            let v = v.to_owned();
            map.insert(k, v);
        }
        Self(map)
    }
}

impl<T> DebugResponse<T>
where
    T: serde::Serialize + Default,
{
    pub fn with_headers(mut self, headers: &HeaderMap<HeaderValue>) -> Self {
        self.headers = QueryObject::from(headers);
        self
    }
    pub fn with_context(mut self, context: &Context) -> Self {
        self.context = serde_json::to_value(context).unwrap();
        self
    }
    pub fn with_method<M>(mut self, method: M) -> Self
    where
        M: AsRef<str>,
    {
        self.method = method.as_ref().to_owned();
        self
    }
    pub fn with_path<S>(mut self, path: S) -> Self
    where
        S: Into<String>,
    {
        self.path_raw = path.into();
        self.path_params = self
            .path_raw
            .split("/")
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .collect::<Vec<String>>();
        self
    }
    pub fn with_query(mut self, query: &QueryMap) -> Self {
        self.query = QueryObject::from(query);
        self
    }
    pub fn with_payload(mut self, payload: Option<T>) -> Self {
        self.payload = payload;
        self
    }
    pub fn new() -> Self {
        Default::default()
    }
}
