mod game;
mod ui;
mod view;

fn main() {
    let mut siv = cursive::default();
    ui::show_menu_main(&mut siv);
    siv.set_autorefresh(true);
    //siv.set_fps(15);
    siv.run();
}
