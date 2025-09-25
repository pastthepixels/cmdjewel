pub const CONFIG_PATH: &str = "cmdjewel/config.toml";

pub mod strings {
    pub const CMDJEWEL_LOGO: &str = "
               ,   .                _.
  __  ,   ,  _.| __.  __  ,   ,  __  |
 /  ' |\\ /| /  |   | /__' | , | /__' |
 \\__, | ' | \\_,|   , \\__, \\/ \\/ \\__, ',_
                 -'
";
    pub const LOGO_GEMS: &str = "
       ●   ◆   ⬟   ▼   ■   ⬢   ▲
";
    pub const CLASSIC: &str = "Classic";
    pub const CLASSIC_DESC: &str =
        "A classic game of cmdjewel. Match 3 (or more) gems in a row until you run out of moves.";
    pub const ZEN: &str = "Zen";
    pub const ZEN_DESC: &str = "Like Classic, but you can't run out of moves.";
    pub const MSG_WELCOME: &str =
        "Welcome to cmdjewel!\nUse the arrow keys and enter to move around.";
    pub const MAIN_MENU: &str = "Main Menu";
    pub const QUIT: &str = "Quit";
    pub const PLAY: &str = "Play Game";
    pub const LEVEL: &str = "Level";
    pub const SCORE: &str = "Score";
    pub const HINT: &str = "Hint";
    pub const COMMAND: &str = "Command";
    pub const CMD_NOT_FOUND: &str =
        "Command not found. Available commands are main/m, play/p [classic/zen], q[a/!], hint/h";
    pub const KEY_NOT_FOUND: &str =
        "Key not recognized. Use the arrow keys to move and the enter key to enter SWAP mode.";
    pub const SETTINGS: &str = "Settings";
    pub const BACK: &str = "Back";

    pub fn first_save(path: &str) -> String {
        format!("cmdjewel just created a save file at {}. When you return, it'll load your game from that path.", path)
    }

    pub fn game_over(score: u32, level: u8) -> String {
        format!(
            "Game over! You scored {} points and got to level {}.",
            score, level
        )
    }
}

pub mod gems {
    use cmdjewel_core::gems::{Gem, GemColor};
    use cursive::style::{Color, ColorStyle};

    /// Gets a printable string from a game::Gems.
    /// This doesn't belong in board as that file only contains game logic and nothing user-facing.
    pub fn gem_string(gem: Gem) -> String {
        match gem {
            Gem::Empty => "•",
            Gem::Normal(x) => match x {
                GemColor::Blue => "▼",
                GemColor::White => "●",
                GemColor::Red => "■",
                GemColor::Yellow => "◆",
                GemColor::Green => "⬟",
                GemColor::Orange => "⬢",
                GemColor::Purple => "▲",
            },
            Gem::Flame(x) => match x {
                GemColor::Blue => "▽",
                GemColor::White => "○",
                GemColor::Red => "□",
                GemColor::Yellow => "◇",
                GemColor::Green => "⬠",
                GemColor::Orange => "⬡",
                GemColor::Purple => "△",
            },
            Gem::Star(_) => "★",
            Gem::Supernova(_) => "☆",
            Gem::Hypercube(_) => "◩",
        }
        .into()
    }

    /// Gets a ColorStyle given a game::Gems
    pub fn gem_color(gem: Gem) -> ColorStyle {
        match gem {
            Gem::Empty => ColorStyle::new(Color::Rgb(67, 76, 94), Color::Rgb(46, 52, 64)),
            Gem::Normal(x) => colorstyle_from_gemcolor(x),
            Gem::Flame(x) => colorstyle_from_gemcolor(x),
            Gem::Star(x) => colorstyle_from_gemcolor(x),
            Gem::Supernova(x) => colorstyle_from_gemcolor(x),
            Gem::Hypercube(_) => ColorStyle::new(Color::Rgb(213, 219, 230), Color::Rgb(67, 76, 94)),
        }
    }

    /// Returns a ColorStyle from a game::GemColor
    fn colorstyle_from_gemcolor(gem_color: GemColor) -> ColorStyle {
        match gem_color {
            GemColor::Blue => ColorStyle::new(Color::Rgb(126, 158, 189), Color::Rgb(46, 52, 64)),
            GemColor::White => ColorStyle::new(Color::Rgb(213, 219, 230), Color::Rgb(46, 52, 64)),
            GemColor::Red => ColorStyle::new(Color::Rgb(190, 96, 105), Color::Rgb(46, 52, 64)),
            GemColor::Yellow => ColorStyle::new(Color::Rgb(233, 201, 138), Color::Rgb(46, 52, 64)),
            GemColor::Green => ColorStyle::new(Color::Rgb(162, 188, 139), Color::Rgb(46, 52, 64)),
            GemColor::Orange => ColorStyle::new(Color::Rgb(207, 135, 111), Color::Rgb(46, 52, 64)),
            GemColor::Purple => ColorStyle::new(Color::Rgb(174, 174, 255), Color::Rgb(46, 52, 64)),
        }
    }
}
