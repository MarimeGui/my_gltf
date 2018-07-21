pub type BufferViews = Vec<BufferView>;

#[derive(Serialize, Deserialize)]
pub struct BufferView {
    pub buffer: usize,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename = "byteOffset")]
    pub byte_offset: Option<usize>,
    #[serde(rename = "byteLength")]
    pub byte_length: usize,
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename = "byteStride")]
    pub byte_stride: Option<usize>
}