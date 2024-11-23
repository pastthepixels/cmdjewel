use crate::game::BoardConfig;
use crate::music::ModulePlayer;
// Handles game UI.
use crate::view::BoardView;
use cursive::event::Event;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Button, Dialog, EditView, LinearLayout, NamedView, OnEventView, Panel, ProgressBar, TextView,
};
use cursive::Cursive;

// Menus
pub fn show_menu_main(s: &mut Cursive) {
    s.pop_layer();
    // Soundtrack
    let module_player: &mut ModulePlayer = s.user_data().unwrap();
    module_player.module.set_pattern(0x02);
    // Creates a button list
    let buttons = LinearLayout::vertical()
        .child(Button::new("Classic", |s| {
            show_game(s, BoardConfig::new_classic());
        }))
        .child(Button::new("Zen", |s| {
            show_game(s, BoardConfig::new_zen());
        }));
    // Adds the dialog
    s.add_layer(
        Dialog::around(buttons)
            .title("welcome to cmdjewel")
            .button("Quit", |s| s.quit()),
    );
}

// Game
pub fn show_game(s: &mut Cursive, config: BoardConfig) {
    s.pop_layer();
    // Soundtrack
    let module_player: &mut ModulePlayer = s.user_data().unwrap();
    module_player.module.set_pattern(0x0D); // B0D
    let name = config.name.clone();
    // Creates the layout for the dialog
    let layout = LinearLayout::vertical()
        .child(
            LinearLayout::horizontal()
                .child(cursive::views::PaddedView::lrtb(
                    1,
                    1,
                    1,
                    1,
                    LinearLayout::vertical()
                        .child(NamedView::new("level", TextView::new("Level X")))
                        .child(NamedView::new("score", TextView::new("XXXXX")))
                        .child(TextView::new("\n")) // TODO: this is the worst way to do a margin wtf
                        .child(Button::new("Hint", |s| {
                            s.call_on_name("board", |view: &mut BoardView| view.hint());
                            // Highlights the game window
                            s.focus_name("board").expect("could not focus");
                        }))
                        .child(LinearLayout::vertical().child(Button::new("Quit", show_menu_main))),
                ))
                .child(Panel::new(NamedView::new("board", BoardView::new(config)))),
        )
        .child(cursive::views::PaddedView::lrtb(
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

// Commands
pub fn init_commands(s: &mut Cursive) {
    s.add_global_callback(':', |s| {
        let mut edit_view = EditView::new().on_submit(|s: &mut Cursive, command: &str| {
            // Be generous and trim/lowercase commands
            let command = command.trim().to_lowercase();
            s.pop_layer();
            // Animation debugging
            if command == "explode" {
                s.call_on_name("board", |view: &mut BoardView| {
                    view.animation_explode()
                });
            }
            else if command == "warp" {
                 s.call_on_name("board", |view: &mut BoardView| {
                    view.animation_warp()
                });
            }
            // Other debugging
            else if command == "autoplay" {
                 s.call_on_name("board", |view: &mut BoardView| {
                     view.autoplay = !view.autoplay;
                });
            }
            else if command == "noanims" {
                 s.call_on_name("board", |view: &mut BoardView| {
                     view.animations_enabled = !view.animations_enabled;
                });
            }
            else if command == "dbgstats" {

                 let debug_string = s.call_on_name("board", |view: &mut BoardView| {
                view.get_debug()
                 }).unwrap();

        s.add_layer(Dialog::info(&debug_string));
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
            }
            else if command == "play zen" || command == "p zen" {
                show_game(s, BoardConfig::new_zen());
            }
            // Vim keys
            else if command == "q"
                || command == "qa"
                || command == "q!"
                || command == "qa!"
            {
                s.quit();
            } else if command == "h" || command == "hint" {
                s.call_on_name("board", |view: &mut BoardView| view.hint());
            } else {
            // In case nothing was recognized, display a help window.
            s.add_layer(Dialog::info("Command not found. Available commands are main/m, play/p [classic/zen], q[a/!], hint/h"));
            }
        });
        edit_view.set_filler(" ");
        s.add_layer(
            Dialog::new().title("Command").content(OnEventView::new(LinearLayout::horizontal().child(TextView::new("> ")).child(edit_view.full_width())).on_event(Event::Key(cursive::event::Key::Esc), |s| {
                s.pop_layer();
            }))
                .with_name("command")
                .fixed_width(32),
        );
    });
}
