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
    context: Context,
    path_params: Vec<String>,
    payload: Option<T>,
}

/// Wrapper type to allow for serialization of HeaderMap and QueryMap
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
        Self(HashMap::from_iter(hm.iter().map(|(k, v)| {
            (
                k.as_str().to_owned(),
                String::from_utf8_lossy(v.as_bytes()).into_owned(),
            )
        })))
    }
}

impl From<&QueryMap> for QueryObject {
    fn from(qm: &QueryMap) -> Self {
        Self(HashMap::from_iter(
            qm.iter().map(|(k, v)| (k.to_owned(), v.to_owned())),
        ))
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
        self.context = context.clone();
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
