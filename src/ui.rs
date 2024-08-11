// Handles game UI.
use crate::view::BoardView;
use cursive::views::{Button, Dialog, LinearLayout, Panel};
use cursive::Cursive;

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
        .child(Panel::new(
            LinearLayout::vertical()
                .child(Button::new("Hint", |_| {}))
                .child(LinearLayout::vertical().child(Button::new("Quit", show_menu_main))),
        ))
        .child(Panel::new(BoardView::new()));
    // Creates the dialog
    let mut game_dialog = Dialog::around(layout).title("cmdjewel");
    // Grabs focus of the board
    game_dialog
        .get_content_mut()
        .take_focus(cursive::direction::Direction::right())
        .expect(":(");
    // Adds the dialog into a new layer
    s.add_layer(game_dialog);
}

// Events
