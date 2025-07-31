![cmdjewel](assets/images/logo.svg)
========

[cmdjewel.webm](https://github.com/user-attachments/assets/4173f720-140a-40ca-bd05-ac8a7674ebf0)

cmdjewel is a terminal match-3 game inspired by Bejeweled, written with Rust and cursive.

# Running

To run cmdjewel from source, you need to have cargo installed. The recommended way of installing it is through [rustup](https://rustup.rs/).

Next, you'll need `libopenmpt`--you can install it on any Linux distribution.

After cloning cmdjewel, type `cargo run` in a terminal.

# Playing

cmdjewel uses a modal control system. To navigate in SELECT mode, use the arrow keys. Hit space to enter SWAP mode where you can swap a piece with any adjacent
one using the arrow keys. Alternatively you can use Vim keybinds (h, j, k, l) by default.

If you notice everything's too small, try changing your terminal's font size. The game is designed to run at any font size (as long as everything fits!)

# TODO:
- [ ] MacOS export
- [ ] Music/SFX slider
- [ ] Settings page accessible in-game/main menu
- [ ] Saving/loading from config files (TOML probably somewhere in ~/.config)
- [ ] Better title screen logo
- [ ] Animated title screen background
- [ ] Animation from moving from the title screen to main menu
- [ ] Main menu resembling Bejeweled 3
- [ ] Sound effects
- [ ] Special gems that explode hypercubes activate those hypercubes
- [x] Hypercube with hypercube matching
- [ ] Star gems
- [ ] Supernova gems
