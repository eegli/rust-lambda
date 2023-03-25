#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct Payload {
    pub name: String,
    pub age: Option<u8>,
}
