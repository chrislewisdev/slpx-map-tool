use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MapElement {
    #[serde(rename = "@width")]
    pub width: u16,
    #[serde(rename = "@height")]
    pub height: u16,
    #[serde(rename = "@tilewidth")]
    pub tile_width: u16,
    #[serde(default)]
    pub layer: Vec<LayerElement>,
    #[serde(default, rename = "objectgroup")]
    pub object_groups: Vec<ObjectGroupElement>,
}

#[derive(Debug, Deserialize)]
pub struct LayerElement {
    #[serde(rename = "@name")]
    pub name: String,
    pub data: DataElement,
}

#[derive(Debug, Deserialize)]
pub struct DataElement {
    #[serde(rename = "$text")]
    pub content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ObjectGroupElement {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(default)]
    pub object: Vec<ObjectElement>,
}

#[derive(Debug, Deserialize)]
pub struct PropertyElement {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@value")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct PropertiesElement {
    #[serde(default, rename = "property")]
    pub properties: Vec<PropertyElement>,
}

fn default_dimension() -> f32 { 0.0 }

#[derive(Debug, Deserialize)]
pub struct ObjectElement {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@type")]
    pub type_id: Option<String>,
    #[serde(rename = "@x")]
    pub x: f32,
    #[serde(rename = "@y")]
    pub y: f32,
    #[serde(rename = "@width", default = "default_dimension")]
    pub width: f32,
    #[serde(rename = "@height", default = "default_dimension")]
    pub height: f32,
    pub properties: Option<PropertiesElement>,
}
