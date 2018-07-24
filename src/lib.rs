extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod accessors;
pub mod asset;
pub mod buffer_views;
pub mod buffers;
pub mod materials;
pub mod meshes;
pub mod nodes;
pub mod scenes;
pub mod skins;
pub mod textures;
pub mod images;

use serde_json::Error;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct GlTF {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessors: Option<accessors::Accessors>,
    pub asset: asset::Asset,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffers: Option<buffers::Buffers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "bufferViews")]
    pub buffer_views: Option<buffer_views::BufferViews>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<images::Images>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub materials: Option<materials::Materials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meshes: Option<meshes::Meshes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<nodes::Nodes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scenes: Option<scenes::Scenes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skins: Option<skins::Skins>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub textures: Option<textures::Textures>,
}

impl GlTF {
    pub fn write_gltf<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        serde_json::to_writer(writer, self)
    }
    pub fn write_gltf_pretty<W: Write>(&self, writer: &mut W) -> Result<(), Error> {
        serde_json::to_writer_pretty(writer, self)
    }
    pub fn get_json(&self) -> Result<String, Error> {
        serde_json::to_string_pretty(self)
    }
}
