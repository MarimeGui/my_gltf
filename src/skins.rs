pub type Skins = Vec<Skin>;

#[derive(Serialize, Deserialize)]
pub struct Skin {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "inverseBindMatrices")]
    pub inverse_bind_matrices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skeleton: Option<usize>,
    pub joints: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
