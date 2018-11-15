pub type Images = Vec<Image>;

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
}
