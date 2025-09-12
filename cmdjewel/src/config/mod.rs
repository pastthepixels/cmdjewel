use std::fs::File;
use std::io::Read;
use cmdjewel_core::board::{Board, BoardConfig};
use cmdjewel_core::gems::Gem;

pub mod data;

/// Loads a config file and returns a corresponding Config struct.
/// If:
///     1. There is no config file
///     2. There is a config file, but it can't be parsed
/// return data::Config::default()
pub fn load_config() -> data::Config {
    if let Some(dir) = dirs::config_local_dir() {
        // Load cmdjewel config file
        let dir = dir.join("cmdjewel/config.toml"); // TODO: string constant
        // If it exists, read it.
        if dir.exists() {
            let file = File::open(dir);
            let mut data = String::new();
            file.unwrap()
                .read_to_string(&mut data)
                .expect("cmdjewel config file exists, but can't be read");
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

/// Loads a config file and gets music_vol (or 1 by default).
pub fn get_music_vol() -> f32 {
    load_config().settings.music_vol
}

/// Saves a board to an existing Save. Requires get_save() to return a valid save
pub fn save_board(board: &Board, is_game_over: bool) {
    // Load the config (or the default config)
    let mut cfg = load_config();
    // Create a game save from the board
    let gs = if !is_game_over {
        Some(data::GameSave::new(&board)   )
    } else {
        None
    };
    // Update the save, store
    // TODO: hack to determine zen mode/classic mode. implement boardconfig::gamemode : enum
    if board.config_ref().infinite {
        cfg.save.zen = gs;
    } else {
        cfg.save.classic = gs;
    }
    // Write to config file
    if let Some(dir) = dirs::config_local_dir() {
        let dir = dir.join("cmdjewel/config.toml"); // TODO: string constant
        std::fs::write(dir, toml::to_string_pretty(&cfg).unwrap()).unwrap(); // TODO: unwraps, is this guaranteed to succeed?
    }
}


/// Creates a new Board. If a save exists for its gamemode, loads the save. Otherwise, creates a new Board.
pub fn board(config: BoardConfig) -> Board {
    // Load the config (or the default config)
    let cfg = load_config();
    // Get game save
    // TODO: hack to determine zen mode/classic mode. implement boardconfig::gamemode : enum
    let gs = if config.infinite {
        cfg.save.zen
    } else {
        cfg.save.classic
    };
    // Create Board
    if let Some(save) = gs {
        // TODO: wow this code sucks
        let mut data = [Gem::Empty; 64];
        let save_data = save.data.iter().map(|g| {g.0}).collect::<Vec<Gem>>();
        data.copy_from_slice(&save_data.as_slice());
        Board::new_controlled(config, data, save.score, save.level, save.level_progress)
    } else {
        Board::new(config)
    }
}