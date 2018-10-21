use std::collections::BTreeMap;

pub type Meshes = Vec<Mesh>;

#[derive(Serialize, Deserialize)]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

#[derive(Serialize, Deserialize)]
pub struct Primitive {
    pub attributes: BTreeMap<String, usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<usize>
}
