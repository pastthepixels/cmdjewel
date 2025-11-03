mod animations;
mod config;
mod constants;
mod ui;
mod view;

use cpal::traits::StreamTrait;
use cursive::{theme::Color, traits::With};

fn main() {
    let mut siv = cursive::default();
    // Set a theme (nord) (I copy and pasted this from the git repo and changed some colors)
    siv.set_theme(cursive::theme::Theme {
        shadow: false,
        borders: cursive::theme::BorderStyle::Simple,
        palette: cursive::theme::Palette::retro().with(|palette| {
            use cursive::style::PaletteColor::*;

            palette[Background] = Color::Rgb(46, 52, 64);
            palette[View] = Color::Rgb(46, 52, 64);
            palette[Primary] = Color::Rgb(213, 219, 230);
            palette[HighlightText] = Color::Rgb(213, 219, 230);
            palette[TitlePrimary] = Color::Rgb(126, 158, 189);
            palette[Secondary] = Color::Rgb(76, 86, 106);
            palette[Tertiary] = Color::Rgb(59, 66, 82);
            palette[Highlight] = Color::Rgb(190, 96, 105);
            palette[HighlightInactive] = Color::Rgb(126, 158, 189);
            palette.set_color("BoardInactive", Color::Rgb(59, 66, 82));
            palette.set_color("GemInactive", Color::Rgb(76, 86, 106));
        }),
    });
    // Sets the terminal background--uses an ANSI escape sequence to run an Operating System Command (OSC)
    // that gets picked up by your terminal emulator. XTerm (and other emulators) implemented this
    // as a command to change the background.
    println!("\x1b]11;#2E3440\x07");
    // Set up music
    it2play_rs::load_bytes(
        Vec::from(include_bytes!("../cmdjewel.it")),
        it2play_rs::IT2Driver::HQ,
    );
    let stream = it2play_rs::generate_stream();
    stream.play().unwrap();
    it2play_rs::play(0);
    it2play_rs::set_global_volume((config::load_config().settings.music_vol * 128.) as u16);
    // show the start screen
    ui::show_menu_splash(&mut siv);
    // set up commands
    ui::init_commands(&mut siv);
    // Set the refresh rate to 30 FPS and run
    siv.set_autorefresh(true);
    siv.run();
    // Cleaning up: Reset the background (again on terminals that support it)
    // Konsole (KDE) doesn't support this, but Alacritty and XTerm do. You might find it's a hit-or-miss with your terminal.
    println!("\x1b]111;\x07")
}
