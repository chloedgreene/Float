use serde::Deserialize;

#[derive(Deserialize)]
pub struct WorldManifest{
    name: String,
    maps: MapConfig,
    world: Option<WorldConfig>
}

#[derive(Deserialize)]
pub struct MapConfig{
    depth:String,
    colour:String
}

#[derive(Deserialize)]
pub struct WorldConfig{
    skybox: Option<String>
}