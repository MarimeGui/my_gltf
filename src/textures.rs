pub type Textures = Vec<Texture>;

#[derive(Serialize, Deserialize)]
pub struct Texture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampler: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<usize>,
}
