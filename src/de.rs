use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapElement {
    #[serde(rename = "@width")]
    pub width: u16,
    #[serde(rename = "@height")]
    pub height: u16,
    #[serde(rename = "@tilewidth")]
    pub tile_width: u16,
    #[serde(rename = "@tileheight")]
    pub tile_height: u16,
    #[serde(default)]
    pub layer: Vec<LayerElement>,
    #[serde(default, rename = "objectgroup")]
    pub object_group: Vec<ObjectGroupElement>,
}

#[derive(Debug, Deserialize)]
pub struct LayerElement {
    #[serde(rename = "@id")]
    pub id: u16,
    #[serde(rename = "@name")]
    pub name: String,
    pub data: DataElement,
}

#[derive(Debug, Deserialize)]
pub struct DataElement {
    #[serde(rename = "@encoding")]
    pub encoding: String,
    #[serde(rename = "$text")]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ObjectGroupElement {
    #[serde(rename = "@id")]
    pub id: u16,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub object: Vec<ObjectElement>,
}

#[derive(Debug, Deserialize)]
pub struct ObjectElement {
    #[serde(rename = "@id")]
    pub id: u16,
    #[serde(rename = "@type")]
    pub type_id: String,
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
}
