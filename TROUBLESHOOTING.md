Help, something went wrong!
===========================

## Some characters aren't displaying???

cmdjewel relies on Unicode characters. Try switching to a terminal emulator that doesn't use bitmap fonts.
On Windows, the default command prompt (cmd.exe) won't work. Try installing "Windows Terminal" -- an app from the app store that's installed by default on Windows 11.

On Linux, try Alacritty, kitty, or Konsole (cmdjewel is developed on Alacritty and Konsole).


## cmdjewel crashed for some reason, what now?

Did cmdjewel just randomly stop working? Try running it with `cmdjewel 2>debug.txt` - this pipes the error output to a text file. Recreate the issue and then open a bug report on https://github.com/pastthepixels/cmdjewel with that file. Be sure to tell me what you did to cause it to crash!


## Everything is too small, it's hard to read! Or: Things are too big for my terminal window, some parts of the game are just being clipped off!

Try resizing your terminal window, or readjusting your font size to an adequate size to fit everything. If it can fit the main menu with a bit of space, it can fit everything you'll need. Be sure you see the cmdjewel logo, main menu, and welcome splash when you're changing the size of fonts on your terminal.


## cmdjewel saved a file when I changed a setting (or saved). Where is it?

It'll try to save to your operating system's config directory. On Linux-based systems, that'll be `~/.config/cmdjewel`.
