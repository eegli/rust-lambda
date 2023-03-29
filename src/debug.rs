use lambda_http::{
    aws_lambda_events::query_map::QueryMap,
    http::{HeaderMap, HeaderValue},
    Context,
};

use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct DebugResponse<T>
where
    T: serde::Serialize + Default,
{
    headers: HashMap<String, String>,
    query: HashMap<String, Vec<String>>,
    method: String,
    path_raw: String,
    context: Context,
    path_params: Vec<String>,
    payload: Option<T>,
}

trait ToHashMap<T> {
    fn to_hashmap(&self) -> HashMap<String, T>;
}

impl ToHashMap<String> for HeaderMap<HeaderValue> {
    fn to_hashmap(&self) -> HashMap<String, String> {
        self.iter()
            .map(|(k, v)| {
                (
                    k.as_str().to_owned(),
                    String::from_utf8_lossy(v.as_bytes()).into_owned(),
                )
            })
            .collect()
    }
}

impl ToHashMap<Vec<String>> for QueryMap {
    fn to_hashmap(&self) -> HashMap<String, Vec<String>> {
        self.iter().fold(HashMap::new(), |mut acc, (k, v)| {
            acc.entry(k.to_owned()).or_default().push(v.to_owned());
            acc
        })
    }
}

impl<T> DebugResponse<T>
where
    T: serde::Serialize + Clone + Default,
{
    pub fn with_headers(mut self, headers: &HeaderMap<HeaderValue>) -> Self {
        self.headers = headers.to_hashmap();
        self
    }
    pub fn with_context(mut self, context: Context) -> Self {
        self.context = context;
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
    pub fn with_query(mut self, query: QueryMap) -> Self {
        self.query = query.to_hashmap();
        self
    }
    pub fn with_payload(mut self, payload: &Option<T>) -> Self {
        self.payload = payload.clone();
        self
    }
    pub fn new() -> Self {
        Default::default()
    }
}
