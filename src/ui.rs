use cursive::direction::Direction;
// Handles game UI.
use cursive::event::{Event, EventResult, Key};
use cursive::traits::*;
use cursive::views::{Button, Canvas, Dialog, DummyView, EditView, LinearLayout, SelectView};
use cursive::Cursive;

use crate::game;
use crate::view::BoardView;

/// Cursor modes
enum CursorMode {
    Normal,
    Swap,
}

impl std::fmt::Display for CursorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorMode::Normal => write!(f, "NORMAL"),
            CursorMode::Swap => write!(f, "SWAP"),
        }
    }
}

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
            .title("cmdjewel rewrite (wip)")
            .button("Quit", |s| s.quit()),
    );
}

pub fn show_game(s: &mut Cursive) {
    s.pop_layer();
    // Creates the layout for the dialog
    let layout = LinearLayout::horizontal()
        .child(
            LinearLayout::vertical()
                .child(Button::new("Hint", |_| {}))
                .child(LinearLayout::vertical().child(Button::new("Quit", show_menu_main))),
        )
        .child(BoardView::new());
    // Creates the dialog
    let game_dialog = Dialog::around(layout).title("cmdjewel");
    s.add_layer(game_dialog);
}

// Events
