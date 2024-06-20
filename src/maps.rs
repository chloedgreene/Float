use serde::Deserialize;

pub struct World{
    pub manifest: WorldManifest,
    pub colour_map:Box<[(u8,u8,u8); 1024*1024]>,
    pub depth_map :Box<[u8; 1024*1024]>,
}

#[derive(Deserialize)]
pub struct WorldManifest{
    pub name: String,
    pub maps: MapConfig,
    pub world: Option<WorldConfig>
}

#[derive(Deserialize)]
pub struct MapConfig{
    pub depth:String,
    pub colour:String
}

#[derive(Deserialize)]
pub struct WorldConfig{
    pub skybox: Option<String>
}