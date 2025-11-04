use cmdjewel_core::gems::{Gem, GemColor};
use cursive::{
    view::Resizable,
    views::{Dialog, LinearLayout, PaddedView, Panel, ResizedView, ScrollView, TextView},
    Cursive,
};

use crate::{
    constants::{gems, strings},
    hspacer, vspacer,
};

pub fn show_help(s: &mut Cursive) {
    s.add_layer(
        Dialog::new()
            .title(strings::HELP.to_lowercase())
            .content(
                LinearLayout::vertical()
                    .child(vspacer!())
                    .child(TextView::new(strings::HELP_CMDJEWEL))
                    .child(vspacer!())
                    .child(Panel::new(TextView::new(strings::HELP_CONTROLS)))
                    .child(vspacer!())
                    .child(TextView::new(strings::HELP_TUTORIAL)),
            )
            .button(strings::TUTORIAL, |s| show_tutorial(s))
            .button(strings::TROUBLESHOOTING, |s| show_troubleshooting(s))
            .button(strings::OK, |s| {
                s.pop_layer();
            })
            .max_width(52),
    );
}

pub fn show_tutorial(s: &mut Cursive) {
    s.add_layer(
        Dialog::info("")
            .title(strings::TUTORIAL.to_lowercase())
            .content(
                LinearLayout::vertical()
                    .child(vspacer!())
                    .child(TextView::new(strings::TUTORIAL_1))
                    .child(vspacer!())
                    .child(
                        LinearLayout::horizontal()
                            .child(
                                LinearLayout::vertical()
                                    .child(board(vec![
                                        [
                                            Gem::Normal(GemColor::Red),
                                            Gem::Normal(GemColor::White),
                                            Gem::Normal(GemColor::Blue),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Red),
                                            Gem::Normal(GemColor::Red),
                                            Gem::Normal(GemColor::Blue),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Purple),
                                            Gem::Normal(GemColor::Red),
                                            Gem::Normal(GemColor::White),
                                        ],
                                    ]))
                                    .child(TextView::new(strings::TUTORIAL_2)),
                            )
                            .child(hspacer!())
                            .child(
                                LinearLayout::vertical()
                                    .child(board(vec![
                                        [
                                            Gem::Normal(GemColor::Blue),
                                            Gem::Normal(GemColor::Green),
                                            Gem::Normal(GemColor::White),
                                        ],
                                        [
                                            Gem::Normal(GemColor::White),
                                            Gem::Normal(GemColor::Green),
                                            Gem::Normal(GemColor::Yellow),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Orange),
                                            Gem::Normal(GemColor::Blue),
                                            Gem::Normal(GemColor::Green),
                                        ],
                                        [
                                            Gem::Normal(GemColor::White),
                                            Gem::Normal(GemColor::Green),
                                            Gem::Normal(GemColor::Blue),
                                        ],
                                    ]))
                                    .child(TextView::new(strings::TUTORIAL_3)),
                            )
                            .child(hspacer!())
                            .child(
                                LinearLayout::vertical()
                                    .child(board(vec![
                                        [
                                            Gem::Normal(GemColor::White),
                                            Gem::Normal(GemColor::Purple),
                                            Gem::Normal(GemColor::Green),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Orange),
                                            Gem::Normal(GemColor::Blue),
                                            Gem::Normal(GemColor::Green),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Green),
                                            Gem::Normal(GemColor::Green),
                                            Gem::Normal(GemColor::Blue),
                                        ],
                                        [
                                            Gem::Normal(GemColor::Orange),
                                            Gem::Normal(GemColor::Yellow),
                                            Gem::Normal(GemColor::Green),
                                        ],
                                    ]))
                                    .child(TextView::new(strings::TUTORIAL_4)),
                            ),
                    ),
            )
            .max_width(52),
    );
}

pub fn show_troubleshooting(s: &mut Cursive) {
    s.add_layer(
        Dialog::info("")
            .content(ScrollView::new(TextView::new(include_str!(
                "../../../TROUBLESHOOTING.md"
            ))))
            .title(strings::TROUBLESHOOTING.to_lowercase())
            .full_screen(),
    );
}

fn board(b: Vec<[Gem; 3]>) -> PaddedView<ResizedView<Panel<LinearLayout>>> {
    let size = (11, b.len() + 2);
    let mut layout = LinearLayout::vertical();
    b.iter().for_each(|row| {
        let mut h = LinearLayout::horizontal();
        row.iter().for_each(|gem| {
            h.add_child(
                TextView::new(format!(" {} ", gems::gem_string(*gem))).style(gems::gem_color(*gem)),
            )
        });
        layout.add_child(h);
    });
    return PaddedView::lrtb(1, 0, 1, 7 - size.1, Panel::new(layout).max_size(size));
}
