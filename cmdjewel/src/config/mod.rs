use std::error::Error;
use std::fs::File;
use std::io::Read;

mod data;

/// Loads a config file and returns a new Config struct.
pub fn load_config() -> Option<data::Config> {
    if let Some(dir) = dirs::config_local_dir() {
        // Load cmdjewel config file
        let dir = dir.join("cmdjewel/config.toml");
        // If it exists, read it.
        if dir.exists() {
            let file = File::open(dir);
            let mut data = String::new();
            file.unwrap().read_to_string(&mut data).expect("cmdjewel config file exists, but can't be read");
            Some(toml::from_str::<data::Config>(&data).unwrap())
        } else {
            None
        }
    } else {
        None
    }
}

/// Loads a config file and gets music_vol (or 1 by default).
pub fn get_music_vol() -> f32 {
    if let Some(cfg) = load_config() {
        cfg.settings.music_vol
    } else {
        1.
    }
}

/// Gets the current save--and returns None if it is invalid or doesn't exist.
pub fn get_save() -> Option<data::Save> {
    if let Some(cfg) = load_config() {
        if cfg.save.len() < cfg.save_id {
            Some(cfg.save[cfg.save_id].clone())
        } else {
            None
        }
    } else {
        None
    }
}

/// Gets all saves.
pub fn get_saves() -> Vec<data::Save> {
    todo!();
}


/// Writes a save to config.toml; creates it if it doesn't exist.
/// If the save ID exists, it overwrites it. If the save ID does not exist, it adds it to the list of saves.
pub fn write_save(save: data::Save) {
    todo!();
}