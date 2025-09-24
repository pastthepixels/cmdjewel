use crate::constants;
use cmdjewel_core::board::{Board, BoardConfig, Gamemode};
use cmdjewel_core::gems::Gem;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use toml_edit::visit_mut::VisitMut;

pub mod data;
mod hacks;

/// Loads a config file and returns a corresponding Config struct.
/// If:
///     1. There is no config file
///     2. There is a config file, but it can't be parsed
/// return data::Config::default()
pub fn load_config() -> data::Config {
    if let Some(dir) = dirs::config_local_dir() {
        // Load cmdjewel config file
        let dir = dir.join(constants::CONFIG_PATH);
        // If it exists, read it.
        if dir.exists() {
            let file = File::open(dir);
            let mut data = String::new();
            file.unwrap().read_to_string(&mut data).unwrap();
            // Reads the config file, returns data::Config::default() if it can't deserialize
            toml::from_str::<data::Config>(&data).unwrap_or(data::Config::default())
        } else {
            // Config file doesn't exist
            data::Config::default()
        }
    } else {
        // Unable to get config directory
        data::Config::default()
    }
}

/// Returns the full path of a config file
pub fn config_path() -> Option<PathBuf> {
    if let Some(dir) = dirs::config_local_dir() {
        Some(dir.join(constants::CONFIG_PATH))
    } else {
        None
    }
}

/// Saves a board to an existing Save. Requires get_save() to return a valid save
pub fn save_board(board: &Board, is_game_over: bool) {
    // Load the config (or the default config)
    let mut cfg = load_config();
    // Create a game save from the board
    let gs = if !is_game_over {
        Some(data::GameSave::new(&board))
    } else {
        None
    };
    // Update the save, store
    match board.config_ref().gamemode {
        Gamemode::ZEN => cfg.save.zen = gs,
        Gamemode::CLASSIC => cfg.save.classic = gs,
    };
    // Write to config file
    if let Some(dir) = dirs::config_local_dir() {
        let dir = dir.join(constants::CONFIG_PATH);
        let mut doc = toml_edit::ser::to_document(&cfg).unwrap();
        let mut visitor = hacks::HackyFormatter;
        // Write save
        {
            let mut folder = dir.clone();
            folder.pop();
            std::fs::create_dir(folder).unwrap_or_default();
        }
        visitor.visit_document_mut(&mut doc);
        std::fs::write(dir, doc.to_string()).unwrap();
    }
}

/// Creates a new Board. If a save exists for its gamemode, loads the save. Otherwise, creates a new Board.
pub fn new_board(config: BoardConfig) -> Board {
    // Load the config (or the default config)
    let cfg = load_config();
    // Get game save
    let gs = match config.gamemode {
        Gamemode::ZEN => cfg.save.zen,
        Gamemode::CLASSIC => cfg.save.classic,
    };
    // Create Board
    if let Some(save) = gs {
        // TODO: wow this code sucks
        let mut data = [Gem::Empty; 64];
        let save_data = save.data.iter().map(|g| g.0).collect::<Vec<Gem>>();
        data.copy_from_slice(&save_data.as_slice());
        Board::new_controlled(config, data, save.score, save.level, save.level_progress)
    } else {
        Board::new(config)
    }
}
