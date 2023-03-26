#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Default)]
pub struct Payload {
    pub name: String,
    pub age: Option<u8>,
}
