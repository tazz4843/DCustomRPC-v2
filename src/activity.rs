use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Activity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<Timestamps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub assets: Option<Assets>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub party: Option<Party>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Timestamps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Assets {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_text: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Party {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<[u32; 2]>,
}
