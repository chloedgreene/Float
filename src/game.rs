use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameConfig{
    pub initial_map: String
}