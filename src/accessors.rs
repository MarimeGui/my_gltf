pub type Accessors = Vec<Accessor>;

#[derive(Serialize, Deserialize)]
pub struct Accessor {
    #[serde(skip_serializing_if="Option::is_none")]
    #[serde(rename = "bufferView")]
    pub buffer_view: Option<usize>,
    #[serde(rename = "componentType")]
    pub component_type: u16,
    pub count: usize,
    #[serde(rename = "type")]
    pub attribute_type: String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<Vec<f32>>,
    #[serde(skip_serializing_if="Option::is_none")]
    pub min: Option<Vec<f32>>
}