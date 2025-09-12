use cpal::Stream;
use cpal::traits::StreamTrait;
use cmdjewel_core::board::BoardConfig;
use crate::constants::strings;
use crate::multiline_button::Button;
use crate::view::BoardView;
use cursive::event::{Callback, Event, EventResult};
use cursive::view::{Margins, Nameable, Resizable};
use cursive::views::{
    Dialog, EditView, FocusTracker, LinearLayout, NamedView, OnEventView, PaddedView, Panel,
    ProgressBar, TextView,
};
use cursive::Cursive;
use crate::config;
use crate::config::save_board;

/// Creates a vertical spacer of size $size, or 1 by default
macro_rules! spacer {
    () => {
        spacer!(1)
    };

    ($size:expr) => {{
        let n: usize = $size; // Force types to be unsigned integers
        TextView::new("\n".repeat(n))
    }};
}

/// Creates a gamemode button for the main menu, that changes `about_gamemode` when focused.
macro_rules! gamemode_btn {
    ($label:expr, $desc:expr, $cb:expr) => {
        FocusTracker::new(Button::new_raw(
            "╭───────────╮\n│".to_string()
                + format!("{: ^11}", $label).as_str()
                + "│\n╰───────────╯",
            $cb,
        ))
        .on_focus(|_| {
            EventResult::Consumed(Some(Callback::from_fn(|s| {
                s.call_on_name("about_gamemode", |view: &mut TextView| {
                    view.set_content($desc)
                });
            })))
        })
    };
}

/// Shows the main menu, where gamemodes can be selected.
/// It's a remake of a combination of Bejeweled 3's "Play" screen and its gamemode selector.
pub fn show_menu_main(s: &mut Cursive) {
    // If a game exists, save it
    s.call_on_name("board", |b: &mut BoardView| { save_board(&b.board, false)}).unwrap_or_default();
    // Remove top layer
    s.pop_layer();
    // Soundtrack
    it2play_rs::play(0x02);
    it2play_rs::set_global_volume((config::get_music_vol() * 128.) as u16);
    // Creates a button list
    let button_classic = gamemode_btn!(strings::CLASSIC, strings::CLASSIC_DESC, |s| {
        show_game(s, BoardConfig::new_classic());
    });
    let button_zen = gamemode_btn!(strings::ZEN, strings::ZEN_DESC, |s| {
        show_game(s, BoardConfig::new_zen());
    });
    let buttons = LinearLayout::vertical()
        .child(button_classic)
        .child(spacer!())
        .child(button_zen);
    // Adds buttons in the main menu, and a descriptor of game modes (when hovered)
    s.add_layer(
        LinearLayout::vertical()
            .child(TextView::new(strings::CMDJEWEL_LOGO))
            .child(
                Dialog::around(buttons)
                    .title(strings::MAIN_MENU.to_lowercase())
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
    );
}

/// Shows the start menu, or splash screen.
/// This is a remake of a combination of Bejeweled 3's loading screen and its "Play" screen.
pub fn show_menu_start(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        LinearLayout::vertical()
            .child(TextView::new(
                strings::LOGO_GEMS.to_string() + strings::CMDJEWEL_LOGO,
            ))
            .child(Button::new_raw(strings::PLAY, show_menu_main)),
    );

    // s.reposition_layer(
    //     cursive::views::LayerPosition::FromFront(0),
    //     cursive::XY::absolute(s.screen_size() / 2 - (1, 0)),
    // );
}

/// This starts the game given a BoardConfig (which decides game factors such as if it is in classic/zen mode)
pub fn show_game(s: &mut Cursive, config: BoardConfig) {
    s.pop_layer();
    // Soundtrack
    it2play_rs::play(0x0D);
    it2play_rs::set_global_volume((config::get_music_vol() * 128.) as u16);
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
                            TextView::new(strings::LEVEL.to_string() + "X"),
                        ))
                        .child(NamedView::new(
                            strings::SCORE.to_lowercase(),
                            TextView::new("X"),
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
    s.add_layer(game_dialog);

    s.focus_name("board").unwrap();
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
            // Sound controls
            else if command == "mpause" {
                let stream: &mut Stream = s.user_data().unwrap();
                stream.pause().unwrap();
            } else if command == "mplay" {
                let stream: &mut Stream = s.user_data().unwrap();
                stream.play().unwrap();
            }
            // Vim keys
            else if command == "q" || command == "qa" { // Save and quit
                // If a game exists, save it
                s.call_on_name("board", |b: &mut BoardView| { save_board(&b.board, false) }).unwrap_or_default();
                s.quit();
            } else if command == "q!" || command == "qa!" { // Force quit
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
