// Handles game UI.
use crate::view::BoardView;
use cursive::event::{Event, EventTrigger};
use cursive::utils::Counter;
use cursive::view::{Nameable, Resizable};
use cursive::views::{
    Button, Dialog, EditView, LayerPosition, LinearLayout, NamedView, Panel, ProgressBar, TextView,
};
use cursive::{Cursive, View};

// Menus
pub fn show_menu_main(s: &mut Cursive) {
    s.pop_layer();
    // Creates a button list
    let buttons = LinearLayout::vertical()
        .child(Button::new("Classic", |s| {
            show_game(s);
        }))
        .child(Button::new("Zen", |s| {
            show_game(s);
        }));
    // Adds the dialog
    s.add_layer(
        Dialog::around(buttons)
            .title("welcome to cmdjewel")
            .button("Quit", |s| s.quit()),
    );
}

// Game
pub fn show_game(s: &mut Cursive) {
    s.pop_layer();
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
                .child(Panel::new(NamedView::new("board", BoardView::new()))),
        )
        .child(cursive::views::PaddedView::lrtb(
            1,
            1,
            0,
            0,
            ProgressBar::new().with_name("progress"),
        ));

    // Creates the dialog
    let game_dialog = Dialog::around(layout).title("classic"); // TODO: rename to zen when appropriate

    // Adds the dialog into a new layer
    s.add_layer(game_dialog);

    s.focus_name("board").unwrap();
}

// Commands
pub fn init_commands(s: &mut Cursive) {
    s.add_global_callback(':', |s| {
        s.add_layer(
            Dialog::new().title("Command").content(
                EditView::new()
                    .on_submit(|s: &mut Cursive, command: &str| {
                        if command == "explode" {
                            s.call_on_name("board", |b: &mut BoardView| b.explode());
                        }
                        if command == "q" || command == "qa" || command == "q!" || command == "qa!"
                        {
                            s.quit();
                        }
                        if command == "h" || command == "hint" {
                            s.call_on_name("board", |view: &mut BoardView| view.hint());
                        }
                        s.pop_layer();
                    })
                    .with_name("command")
                    .fixed_width(40),
            ),
        );
    });
}
