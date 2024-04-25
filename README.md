![cmdjewel](logo.svg)
========

[cmdjewel.webm](https://github.com/pastthepixels/cmdjewel/assets/52388215/7d66040f-5730-4c83-a68f-218b78081a37)

cmdjewel is a terminal match-3 game inspired by Bejeweled, written with Python and ncurses.

# Running

cmdjewel is a Python project. To run it, extract the source code to a folder, ensure you have Python installed, and inside the directory, run `python .` in a terminal. You can optionally install PyGame to play soundtrack.

cmdjewel also requires that you have the `curses` module for Python. On Linux, this is installed by default. If you're on Windows, you have to install [windows-curses](https://github.com/zephyrproject-rtos/windows-curses) with pip.

> [!NOTE]
> In the future, I plan on rewriting the code for cmdjewel in a different language, so there's less hassle to make the game "just work".

# Playing

cmdjewel uses a modal control system. To navigate in SELECT mode, use the arrow keys. Hit space to enter SWAP mode where you can swap a piece with any adjacent
one using the arrow keys. Alternatively you can use Vim keybinds (h, j, k, l) by default.

If you notice everything's too small, try changing your terminal's font size. The game is designed to run at any font size (as long as everything fits!)

# Credits

This game uses beyond_the_network.it:
- by Skaven252,
- retrieved from [The Mod Archive](https://modarchive.org/index.php?request=view_by_moduleid&query=156184),
- and under a CC-BY-NC-ND 4.0 license.

cmdjewel also uses the following libraries:
- PyGame
- libmodplug (OpenMPT)
