use cmdjewel_core::{
    board::Board,
    gems::{Gem, GemColor, GemSelector},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Global settings
    pub settings: Settings,
    /// GameSaves
    pub save: Save,
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    /// Music volume between 0 and 1, inclusive
    pub music_vol: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Save {
    pub classic: Option<GameSave>,
    pub zen: Option<GameSave>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameSave {
    pub data: Vec<GemWrapper>,
    pub score: u32,
    pub level: u8,
    pub level_progress: f32,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Config {
            settings: Settings {
                music_vol: 1.0
            },
            save: Save {
                classic: None,
                zen: None
            }
        }
    }
}

impl GameSave {
    pub fn new(board: &Board) -> Self {
        GameSave {
            data: board
                .as_ref()
                .iter()
                .map(|x| GemWrapper(x.clone()))
                .collect::<Vec<GemWrapper>>(),
            score: board.get_score(),
            level: board.get_level(),
            level_progress: board.get_level_progress(),
        }
    }
}

/*
   Redefine gems
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct GemWrapper(#[serde(with = "GemDef")] pub Gem);

/// Types of gems to use.
#[derive(Serialize, Deserialize)]
#[serde(remote = "Gem")]
pub enum GemDef {
    Empty,
    #[serde(with = "GemColorDef")]
    Normal(GemColor),
    #[serde(with = "GemColorDef")]
    Flame(GemColor),
    // Star gems.
    #[serde(with = "GemColorDef")]
    Star(GemColor),
    // Suprnova gems.
    #[serde(with = "GemColorDef")]
    Supernova(GemColor),
    // Hypercube
    #[serde(with = "GemSelectorDef")]
    Hypercube(GemSelector),
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GemColor")]
pub enum GemColorDef {
    Blue,
    White,
    Red,
    Yellow,
    Green,
    Orange,
    Purple,
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "GemSelector")]
pub enum GemSelectorDef {
    #[serde(with = "GemColorDef")]
    Color(GemColor),
    All,
    None,
}
