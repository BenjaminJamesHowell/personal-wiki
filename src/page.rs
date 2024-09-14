use serde::Deserialize;

#[derive(Deserialize)]
pub struct PageFrontMatter {
    #[serde(default)]
    pub categories: Vec<String>,
}
