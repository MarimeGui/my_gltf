pub type Materials = Vec<Material>;

#[derive(Serialize, Deserialize)]
pub struct Material {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr_metallic_roughness: Option<PbrMetallicRoughness>,
}

#[derive(Serialize, Deserialize)]
pub struct PbrMetallicRoughness {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "baseColorTexture")]
    pub base_color_texture: Option<BaseColorTexture>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "metallicFactor")]
    pub metallic_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "roughnessFactor")]
    pub roughness_factor: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseColorTexture {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<usize>,
}
