use serde::Deserialize;
use cmdjewel_core::gems::Gem;

#[derive(Deserialize)]
pub struct Config {
    /// Global settings
    pub settings: Settings,
    /// ID of the current save.
    pub save_id: usize,
    /// List of saves; contains player names and save data.
    pub save: Vec<Save>
}

#[derive(Deserialize)]
pub struct Settings {
    /// Music volume between 0 and 1, inclusive
    pub music_vol : f32
}

#[derive(Deserialize)]
pub struct Save {
    /// Name of the player that the save belongs to.
    pub name: String,
    /// Save ID
    pub id: usize,
    // Game save data for the Classic gamemode.
    //pub save_classic: Option<[Gem; 64]>
}