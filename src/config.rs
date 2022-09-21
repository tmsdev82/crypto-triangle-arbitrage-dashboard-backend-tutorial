use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct TriangleConfig {
    pub parts: [String; 3],
    pub pairs: [String; 3],
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub update_interval: u32,
    pub results_limit: u32,
    pub depth_streams: Vec<String>,
    pub triangles: Vec<TriangleConfig>,
}
