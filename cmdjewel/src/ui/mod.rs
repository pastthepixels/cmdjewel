use crate::constants::strings;
use crate::ui::multiline_button::Button;
use crate::view::BoardView;
use crate::{config, confirm, gamemode_btn, hspacer, vspacer};
use cmdjewel_core::board::BoardConfig;
use cursive::event::Callback;
use cursive::event::{Event, EventResult};
use cursive::style::PaletteColor;
use cursive::theme::Color;
use cursive::view::{Margins, Nameable, Resizable};
use cursive::views::{
    Dialog, DummyView, EditView, FocusTracker, LayerPosition, LinearLayout, NamedView, OnEventView,
    PaddedView, Panel, ProgressBar, SliderView, TextView,
};
use cursive::{Cursive, View, XY};

mod macros;
mod multiline_button;
pub mod tutorial;

/// Shows the main menu, where gamemodes can be selected.
/// It's a remake of a combination of Bejeweled 3's "Play" screen and its gamemode selector.
pub fn show_menu_main(s: &mut Cursive) {
    // If a game exists, save it
    let mut save_path = None;
    if let Some(p) = config::config_path() {
        // It's possible to get the config path e.g. the OS config path exists
        if !p.exists() && s.find_name::<BoardView>("board").is_some() {
            save_path = Some(p.as_os_str().to_str().unwrap().to_string())
        }
        s.call_on_name("board", move |b: &mut BoardView| {
            config::save_board(&b.board, !b.board.is_valid())
        })
        .unwrap_or_default();
    }
    // Creates a button list
    let button_classic = gamemode_btn!(strings::CLASSIC, strings::CLASSIC_DESC, |s| {
        show_game(s, BoardConfig::new_classic());
    });
    let button_zen = gamemode_btn!(strings::ZEN, strings::ZEN_DESC, |s| {
        show_game(s, BoardConfig::new_zen());
    });
    let buttons = PaddedView::lrtb(
        5,
        0,
        0,
        0,
        LinearLayout::horizontal()
            .child(button_classic)
            .child(hspacer!(2))
            .child(button_zen),
    );
    // Adds buttons in the main menu, and a descriptor of game modes (when hovered)
    switch_screen(
        s,
        LinearLayout::vertical()
            .child(TextView::new(strings::CMDJEWEL_LOGO))
            .child(
                Dialog::around(buttons)
                    .title(strings::MAIN_MENU.to_lowercase())
                    .button(strings::HELP, |s| tutorial::show_help(s))
                    .button(strings::SETTINGS, |s| show_settings(s))
                    .button(strings::QUIT, |s| s.quit())
                    .padding(Margins::lrtb(0, 0, 1, 0)),
            )
            .child(Panel::new(
                PaddedView::lrtb(
                    1,
                    1,
                    0,
                    0,
                    NamedView::new("about_gamemode", TextView::new(strings::MSG_WELCOME)),
                )
                .min_height(3),
            ))
            .max_width(40),
        0x02,
    );
    // Show info dialog if the game is being saved for the first time
    if let Some(path) = save_path {
        s.add_layer(Dialog::info(strings::first_save(&path)));
    }
}

/// Shows the start menu, or splash screen.
/// This is a remake of a combination of Bejeweled 3's loading screen and its "Play" screen.
pub fn show_menu_splash(s: &mut Cursive) {
    switch_screen(
        s,
        LinearLayout::vertical()
            .child(TextView::new(
                strings::LOGO_GEMS.to_string() + strings::CMDJEWEL_LOGO,
            ))
            .child(Button::new_raw(strings::PLAY, show_menu_main)),
        0,
    );
}

/// This starts the game given a BoardConfig (which decides game factors such as if it is in classic/zen mode)
pub fn show_game(s: &mut Cursive, config: BoardConfig) {
    let name = config.name.clone();
    // Creates the layout for the dialog
    let layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(PaddedView::lrtb(
                    1,
                    1,
                    1,
                    1,
                    LinearLayout::vertical()
                        .child(NamedView::new(
                            strings::LEVEL.to_lowercase(),
                            TextView::new(strings::LEVEL.to_string() + "█"),
                        ))
                        .child(NamedView::new(
                            strings::SCORE.to_lowercase(),
                            TextView::new("█"),
                        ))
                        .child(TextView::new("\n")) // TODO: this is the worst way to do a margin wtf
                        .child(Button::new(strings::HINT, |s| {
                            s.call_on_name("board", |view: &mut BoardView| view.hint());
                            // Highlights the game window
                            s.focus_name("board").expect("could not focus");
                        }))
                        .child(
                            LinearLayout::vertical()
                                .child(Button::new(strings::QUIT, show_menu_main)),
                        ),
                ))
                .child(Panel::new(NamedView::new("board", BoardView::new(config)))),
        )
        .child(PaddedView::lrtb(
            1,
            1,
            0,
            0,
            ProgressBar::new().with_name("progress"),
        ));

    // Creates the dialog
    let game_dialog = Dialog::around(layout).title(name);

    // Adds the dialog into a new layer
    switch_screen(s, game_dialog, 0x0d);
}

/// Shows the settings dialog.
pub fn show_settings(s: &mut Cursive) {
    let settings = config::load_config().settings;
    let mut slider = SliderView::horizontal(25); // TODO 25 is a constant
    slider.set_value((settings.music_vol * (slider.get_max_value() - 1) as f32) as usize);
    slider.set_on_change(|_, v| {
        let mut cfg = config::load_config();
        cfg.settings.music_vol = v as f32 / 24.; // TODO 24 is a constant; 25 - 1
        config::save_config(&cfg);
        it2play_rs::set_global_volume((cfg.settings.music_vol * 128.) as u16);
    });
    s.add_layer(
        Dialog::around(
            LinearLayout::vertical().child(
                LinearLayout::horizontal()
                    .child(TextView::new(strings::MUSIC_VOL))
                    .child(hspacer!(2))
                    .child(slider),
            ),
        )
        .title(strings::SETTINGS)
        .button(strings::RESET, |s| {
            confirm!(s, strings::WARN_RESET, |s| {
                config::reset_config();
                s.pop_layer().unwrap();
                s.pop_layer().unwrap();
            })
        })
        .button(strings::BACK, |s| {
            s.pop_layer().unwrap();
        })
        .padding(Margins::lrtb(1, 1, 1, 0)),
    );
}

/// Initialises setting commands by creating a callback for the colon key
pub fn init_commands(s: &mut Cursive) {
    s.add_global_callback(':', |s| {
        let mut edit_view = EditView::new().on_submit(|s: &mut Cursive, command: &str| {
            // Be generous and trim/lowercase commands
            let command = command.trim().to_lowercase();
            s.pop_layer();
            // Animation debugging
            if command == "explode" {
                s.call_on_name("board", |view: &mut BoardView| view.animation_explode());
            } else if command == "warp" {
                s.call_on_name("board", |view: &mut BoardView| view.animation_warp());
            }
            // Other debugging
            else if command == "autoplay" {
                s.call_on_name("board", |view: &mut BoardView| {
                    view.autoplay = !view.autoplay;
                });
            } else if command == "noanims" {
                s.call_on_name("board", |view: &mut BoardView| {
                    view.animations_enabled = !view.animations_enabled;
                });
            }
            // Going to the main menu
            else if command == "main" || command == "m" {
                show_menu_main(s);
            }
            // Starting new games
            else if command == "play"
                || command == "play classic"
                || command == "p"
                || command == "p classic"
            {
                show_game(s, BoardConfig::new_classic());
            } else if command == "play zen" || command == "p zen" {
                show_game(s, BoardConfig::new_zen());
            }
            // Vim keys
            else if command == "q" || command == "qa" {
                // Save and quit
                // If a game exists, save it
                s.call_on_name("board", |b: &mut BoardView| {
                    config::save_board(&b.board, false)
                })
                .unwrap_or_default();
                s.quit();
            } else if command == "q!" || command == "qa!" {
                // Force quit
                s.quit();
            } else if command == "h" || command == "hint" {
                s.call_on_name("board", |view: &mut BoardView| view.hint());
            } else {
                // In case nothing was recognized, display a help window.
                s.add_layer(Dialog::info(strings::CMD_NOT_FOUND));
            }
        });
        edit_view.set_filler(" ");
        s.add_layer(
            Dialog::new()
                .title(strings::COMMAND)
                .content(
                    OnEventView::new(
                        LinearLayout::horizontal()
                            .child(TextView::new("> "))
                            .child(edit_view.full_width()),
                    )
                    .on_event(Event::Key(cursive::event::Key::Esc), |s| {
                        s.pop_layer();
                    }),
                )
                .with_name("command")
                .fixed_width(32),
        );
    });
}

/// Switches the topmost layer with a new layer, `view`. Since screens are displayed on their own layers (e.g. splash screen, main menu screen, games), this effectively fulfills the role of switching screens.
/// We'll use the name "_screen" to denote one of these screens. I'm hesitant to use "scene" as terminology here since we're working with an immediate mode GUI instead of a scene structure like Godot or Unity.
fn switch_screen<T: View>(s: &mut Cursive, view: T, soundtrack: u16) {
    // Switch module order for the screen
    let vol = (config::load_config().settings.music_vol * 128.) as u16;
    it2play_rs::play(soundtrack);
    it2play_rs::set_global_volume(vol);
    // Play an animation! If applicable.
    if let Some(layer_position) = s.screen_mut().find_layer_from_name("_screen") {
        let mut pos = s.screen().layer_offset(layer_position).unwrap();
        let mut ticks = 0;
        let max_ticks = 10;
        let slide = pos.y > max_ticks;
        let palette = s.current_theme().palette.clone();
        s.set_user_data(view);
        s.screen_mut()
            .add_transparent_layer(DummyView::new().with_name("_overlay"));
        s.set_global_callback(Event::Refresh, move |s| {
            // Slide the topmost layer up by 1.
            if slide {
                pos.y -= 1;
            }
            s.reposition_layer(layer_position, XY::absolute(pos));
            // Fade out colors.
            s.update_theme(|t| {
                if let Color::Rgb(br, bg, bb) = PaletteColor::Background.resolve(&t.palette) {
                    PaletteColor::all().for_each(|p| {
                        if let Color::Rgb(r, g, b) = p.resolve(&t.palette) {
                            t.palette[p] =
                                Color::Rgb(r / 2 + br / 2, g / 2 + bg / 2, b / 2 + bb / 2);
                        }
                    });
                }
            });
            // Increase ticks
            ticks += 1;
            // Swap layers, remove callback
            if ticks >= max_ticks {
                let view = s.take_user_data::<T>().unwrap().with_name("_screen");
                while s.pop_layer().is_some() {}
                s.clear();
                s.add_layer(view);
                s.screen_mut().move_to_back(LayerPosition::FromFront(0));
                s.clear_global_callbacks(Event::Refresh);
                // If a game board is found, focus the board.
                s.focus_name("board").unwrap_or(EventResult::Ignored);
                // Reset color palette
                s.update_theme(|t| {
                    t.palette = palette.clone();
                });
            }
        });
    } else {
        // Otherwise just add the view
        s.add_layer(view.with_name("_screen"));
    }
}
