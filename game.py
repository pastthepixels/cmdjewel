#!/usr/bin/env python3
import curses.ascii
import traceback
import random
import curses
import time
import os
from enum import Enum
from math import floor
from board import Board, GEM_TYPES
from console import StatusBar, ProgressBar


ANIM_TIME_FALL = 0.05
ANIM_TIME_ACTION = 0.5
MODES = Enum("MODES", ["select", "swap", "command"])
GEMS = {
    # Blank space listed first
    GEM_TYPES.blank: [" ", 1],
    # Gems!
    GEM_TYPES.diamond: ["▼", 5],
    GEM_TYPES.circle: ["●", 0],
    GEM_TYPES.square: ["■", 2],
    GEM_TYPES.kite: ["◆", 4],
    GEM_TYPES.green: ["◎", 3],
    GEM_TYPES.hexagon: ["⬢", 10],
    GEM_TYPES.triangle: ["▲", 148]
}
KEYBINDS = {
    "left": [curses.KEY_LEFT, ord("h")],
    "right": [curses.KEY_RIGHT, ord("l")],
    "up": [curses.KEY_UP, ord("k")],
    "down": [curses.KEY_DOWN, ord("j")],
    "select": [curses.KEY_ENTER, ord(" ")]
}

class Game:
    """
    Game: how does the user interact with cmdjewel? Handles main menu, and Board instances
    """

    def __init__(self):
        self.running = True
        self.taking_input = True
        self.board = None
        self.cursor = [0, 0]
        # windows
        self.level_bar = None
        self.status_bar = None
        self.stdscr = None
        # tempoary stuff
        self.command = ""
        # TODO remove
        self.autoplay = False

    def __enter__(self):
        # Shortens the curses escape delay
        os.environ.setdefault('ESCDELAY', '15')
        # Initializes the main screen/status bar
        self.stdscr = curses.initscr()
        # curses.newwin(1, self.stdscr.getmaxyx()[1], curses.LINES - 2, 0)
        self.level_bar = ProgressBar(
            screen=self.stdscr, progress=0, foreground_color=3, y=1)
        # curses.newwin(1, self.stdscr.getmaxyx()[1], curses.LINES - 1, 0)
        self.status_bar = StatusBar(screen=self.stdscr, y=0)
        curses.start_color()
        # You don't have to hit the enter key after inputting characters
        curses.cbreak()
        # Don't let characters pressed show back in the terminal
        curses.noecho()
        # No cursor. Let us draw our own.
        curses.curs_set(0)
        # Different way of getting keys that "just works"
        self.stdscr.keypad(True)
        # Forgot what this does but we probably need it.
        self.stdscr.nodelay(True)
        # Initializes color
        curses.use_default_colors()
        for i in range(curses.COLORS):
            curses.init_pair(i + 1, i, - 1)
        # for i in range(255):
        #    self.stdscr.addstr(str(i) + " ", curses.color_pair(i))
        # Begin in SELECT mode
        self.mode = MODES.select
        # Enter game loop:
        #   the game runs as fast as possible and gets input (opposed to updating per input)
        while self.running:
            try:
                # Updates the game/main screen
                self.update()
            except curses.error:
                pass
            except:
                self.running = False
                self.__exit__(None, None, None)
                print(traceback.format_exc())

    def __exit__(self, exc_type, exc_vl, exc_tb):
        self.stdscr.keypad(False)
        curses.nocbreak()
        curses.echo()
        curses.endwin()

    def print(self, *strings: str, end="\n", color=None, reverse=False):
        for string in strings:
            if color is not None:
                self.stdscr.addstr(str(
                    string) + end, curses.color_pair(color) + (curses.A_REVERSE if reverse else 0))
            else:
                self.stdscr.addstr(str(string) + end,
                                   curses.A_REVERSE if reverse else 0)

    def update(self):
        """
        Code to run while you are playing cmdjewel -- not always while you are in a game.
        """
        # Handles input.
        self.update_input()
        # Updates board.
        if self.board:
            self.board.update()
            if self.board.get_game_state() == False:
                self.animation_explode()
                self.board = None
        # Updates windows
        self.update_all_windows()

    def update_all_windows(self):
        self.stdscr.erase()
        if self.board:
            self.update_level_bar()
        self.update_stdscr()
        self.update_status()
        self.stdscr.refresh()


    def update_status(self):
        """
        Updates the status screen with information from the board.
        """
        if self.board != None:
            self.status_bar.set_status_left(" | ".join(self.board.get_status()[:-1]))
            self.status_bar.set_status_right(self.board.get_status()[-1])
        else:
            self.status_bar.set_status_right("")
            self.status_bar.set_status_left("")
        if self.mode == MODES.command:
            self.status_bar.set_status_right("")
            self.status_bar.set_status_left(":" + self.command + "█")
        self.status_bar.update()

    def update_level_bar(self):
        """
        Updates the level progress bar with information from the board.
        """
        self.level_bar.set_progress(self.board.get_level_progress(), 1.)
        self.level_bar.update()

    def update_stdscr(self):
        """
        Update the main screen, with a board if it exists or with a splash screen.
        """
        if self.board:
            self.print_board()
        else:
            self.print("CMDJEWEL\n\nVersion 0\nby PastThePixels\n\n:new classic New classic game\n:new zen New zen game (todo)\n:q Quit cmdjewel")

    def update_input(self):
        """
        Handles input for going into command mode, command mode itself,
        and other modes available once a board is created through update_board_input().

        This can be hell because of how different terminals handle keys.
        As such, we'll try to get Ncurses to do as much as possible.
        """
        key = self.stdscr.getch()
        # Command mode
        if key == ord(":"):
            self.mode = MODES.command
            self.command = ""
        elif self.mode == MODES.command:
            if key == curses.ascii.ESC:
                self.mode = MODES.select
            elif key == curses.KEY_ENTER or key == ord("\n"):
                self.mode = MODES.select
                self.run_command()
            elif key == curses.KEY_BACKSPACE:
                self.command = self.command[:-1]
            elif curses.ascii.isascii(key):
                self.command += chr(key)
        elif self.board and self.taking_input:
            if self.autoplay:
                self.do_autoplay()
            else:
                self.update_board_input(key)

    def update_board_input(self, key):
        """
        Handles input for when you are playing a game.
        If a button is pressed, do a corresponding action on self.board.
        """
        match self.mode:
            case MODES.select:
                if key in KEYBINDS["right"] or key in KEYBINDS["left"]:
                    direction = 1 if key in KEYBINDS["right"] else -1
                    if self.board.is_in_map([self.cursor[0], self.cursor[1] + direction]):
                        self.cursor[1] += direction
                if key in KEYBINDS["up"] or key in KEYBINDS["down"]:
                    direction = 1 if key in KEYBINDS["down"] else -1
                    if self.board.is_in_map([self.cursor[0] + direction, self.cursor[1]]):
                        self.cursor[0] += direction
                # Space to get into select mode.
                if key in KEYBINDS["select"]:
                    self.mode = MODES.swap
                    return

            case MODES.swap:
                # Escape key returns you -- so does another space
                if key == 27 or key in KEYBINDS["select"]:
                    self.mode = MODES.select

                if key in KEYBINDS["right"] or key in KEYBINDS["left"]:
                    self.board.swap("R" if key in KEYBINDS["right"] else "L", self.cursor)
                    self.mode = MODES.select

                if key in KEYBINDS["up"] or key in KEYBINDS["down"]:
                    self.board.swap("U" if key in KEYBINDS["up"] else "D", self.cursor)
                    self.mode = MODES.select


    def animation_explode(self, accel_y = 0.03, velocity=5):
        """
        Plays an animation where the whole board is exploding.
        """
        self.taking_input = False
        self.stdscr.erase()
        gems_on_screen = True
        velocities = [[[random.randint(-velocity, velocity), random.randint(-velocity, velocity)] for i in range(self.board.WIDTH)] for j in range(self.board.HEIGHT)]
        winsize = self.stdscr.getmaxyx()
        t = 0
        while gems_on_screen:
            # Assume gems_on_screen is false unless we prove it is true
            gems_on_screen = False
            self.stdscr.erase()
            for row in range(self.board.HEIGHT):
                for col in range(self.board.WIDTH):
                    # moves cursor
                    velocities[row][col][0] += accel_y
                    position = [velocities[row][col][0] * t, velocities[row][col][1] * t]
                    ncurses_cursor = (
                        row + floor((winsize[0] / 2) - (self.board.HEIGHT / 2)) - 1 + int(position[0]),
                        col*3 + floor((winsize[1] / 2) - (self.board.WIDTH * 3/2)) + int(position[1]) + 1
                    )
                    if winsize[0] > ncurses_cursor[0] >= 0 and winsize[1] > (ncurses_cursor[1]+1) > 0:
                        gems_on_screen = True
                        self.stdscr.move(*ncurses_cursor)
                        # prints gems
                        gem_int = self.board.get_entry(row, col)
                        if gem_int in GEMS:
                            self.print(
                                GEMS[gem_int][0],
                                color=GEMS[gem_int][1],
                                end=""
                            )
            t += 0.05
            time.sleep(0.01)
            self.stdscr.refresh()
        time.sleep(2)
        self.taking_input = True

    def print_board(self):
        """
        Prints the board out. You need a separate function for printing the status.
        :return:
        """
        for row in range(self.board.HEIGHT):
            # Moves the cursor to the desired position
            self.stdscr.move(
                row + floor((self.stdscr.getmaxyx()[0] / 2) - (self.board.HEIGHT / 2)) - 1,
                floor((self.stdscr.getmaxyx()[1] / 2) - (self.board.WIDTH * 3/2))
            )

            # Prints gems
            for col in range(self.board.WIDTH):
                inverted = False
                gem_int = self.board.get_entry(row, col)
                # Checks to see if there is a negative flag saying the number is inverted...
                if gem_int < 1:
                    inverted = True
                    gem_int *= -1
                # Pick a dynamic color per mode, or use the gem's color
                color = GEMS[gem_int][1] if gem_int in GEMS else None
                if row == self.cursor[0] and col == self.cursor[1] and self.taking_input:
                    match self.mode:
                        case MODES.select:
                            color = None

                        case MODES.swap:
                            color = 2
                self.print(
                    " " + str(GEMS[gem_int][0]
                              if gem_int in GEMS else gem_int),
                    end=" ",
                    color=color,
                    reverse=(
                        row == self.cursor[0] and col == self.cursor[1] and self.taking_input) or inverted
                )

    def run_command(self):
        match self.command.split()[0]:
            case "q":
                self.running = False

            case "new":
                self.createBoard()

            case "autoplay":
                self.autoplay = not self.autoplay

            case "explode":
                self.animation_explode()

    def do_autoplay(self):
        """
        Very very inefficient/bad code that just swaps all gems
        until we reach a swappable gem
        """
        for i in range(self.board.HEIGHT):
            for j in range(self.board.WIDTH):
                self.cursor = [i, j]
                if self.board.swap("U", self.cursor) or self.board.swap("D", self.cursor) or self.board.swap("L", self.cursor) or self.board.swap("R", self.cursor):
                    time.sleep(0.1)
                    return
        self.autoplay = False

    def createBoard(self):
        """
        Begins a new game by creating a new board.
        """
        self.board = Board(self)
