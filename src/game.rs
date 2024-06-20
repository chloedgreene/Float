use serde::Deserialize;

#[derive(Deserialize)]
pub struct GameConfig{
    initial_map: String
}