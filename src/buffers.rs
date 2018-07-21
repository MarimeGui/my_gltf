pub type Buffers = Vec<Buffer>;

#[derive(Serialize, Deserialize)]
pub struct Buffer {
    #[serde(skip_serializing_if="Option::is_none")]
    pub uri: Option<String>,
    #[serde(rename = "byteLength")]
    pub byte_length: usize,
}