#!/usr/bin/env python3
import curses.ascii
import traceback
import random
import curses
import time
import os
from enum import Enum
from math import floor, sqrt, sin, cos, atan, pi
from board import Board, GEM_TYPES
from console import StatusBar, ProgressBar

type Vector = list[float]

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
            except:
                self.running = False
                self.__exit__(None, None, None)
                print(traceback.format_exc())

    def __exit__(self, exc_type, exc_vl, exc_tb):
        self.stdscr.keypad(False)
        curses.nocbreak()
        curses.echo()
        curses.endwin()

    # Drawing primitives; text, lines

    def print(self, *strings: str, end="\n", color=None, reverse=False, coords=None, safe=True):
        """
        Prints any amount of strings.

        """
        for string in strings:
            try:
                if coords is not None:
                    self.stdscr.move(*coords)
                if color is not None:
                    self.stdscr.addstr(str(
                        string) + end, curses.color_pair(color) + (curses.A_REVERSE if reverse else 0))
                else:
                    self.stdscr.addstr(str(string) + end,
                                    curses.A_REVERSE if reverse else 0)
            except curses.error as err:
                if not safe:
                    raise err

    def line(self, origin : Vector, target : Vector, color : int = 0):
        """
        Draws a line from an origin vector to a targt vector
        Implemented from the Wikipedia article on Bresenham's line algorithm
        """
        reverse = True
        if color == -1:
            color = 0
            reverse = False

        dx = abs(target[1] - origin[1])
        sx = 1 if origin[0] < target[1] else -1
        dy = -abs(target[0] - origin[0])
        sy = 1 if origin[0] < target[0] else -1
        error = dx + dy

        while True:
            self.print(" ", end="", color=color, reverse=reverse, coords=origin)
            if origin[1] == target[1] and origin[0] == target[0]:
                break
            e2 = 2 * error
            if e2 >= dy:
                if origin[1] == target[1]:
                    break
                error = error + dy
                origin[1] += sx
            if e2 <= dx:
                if origin[0] == target[0]:
                    break
                error = error + dx
                origin[0] += sy



    # Commonly used functions

    def valid(self, y, x):
        """
        checks if a y, x position is valid on the screen
        """
        return 0 <= y < self.stdscr.getmaxyx()[0] and 0 <= x < self.stdscr.getmaxyx()[1]

    # Updates

    def update(self):
        """
        Code to run while you are playing cmdjewel -- not always while you are in a game.
        """
        # Handles input.
        self.update_input()
        # Updates board.
        if self.board:
            previous_level = self.board.get_level()
            self.board.update()
            # Leveling
            if self.board.get_level() > previous_level:
                self.animation_warp()
            # Losing
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

    # Animations

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
                        self.get_gem_position(row, col)[0] + int(position[0]),
                        self.get_gem_position(row, col)[1] + int(position[1])
                    )
                    if self.valid(ncurses_cursor[0], (ncurses_cursor[1]+1)):
                        gems_on_screen = True
                        # prints gems
                        gem_int = self.board.get_entry(row, col)
                        if gem_int in GEMS:
                            self.print(
                                GEMS[gem_int][0],
                                color=GEMS[gem_int][1],
                                end="",
                                coords=ncurses_cursor
                            )
            t += 0.05
            time.sleep(0.01)
            self.stdscr.refresh()
        time.sleep(2)
        self.taking_input = True

    def animation_warp(self, duration_ticks = 100):
        self.taking_input = False
        self.stdscr.erase()
        # 1. Animate circles and pull gems in center.
        # 1a. First we want to define some variables that will help us make the gems spin
        CIRCLE_GROWTH_SPEED = 0.03
        ROTATION_SPEED = 0.8
        GRAVITY = 0.3
        # 2D array, an element is a vector that holds [radius, initial rotation, radial velocity]
        positions = []
        for row in range(self.board.HEIGHT):
            positions.append([])
            for col in range(self.board.WIDTH):
                pos_rel_center = [self.get_gem_position(row, col)[0] - self.stdscr.getmaxyx()[0]/2, self.get_gem_position(row, col)[1] - self.stdscr.getmaxyx()[1]/2]
                positions[row].append([
                    sqrt(pos_rel_center[0] ** 2 + pos_rel_center[1] ** 2),
                    atan(pos_rel_center[0]/pos_rel_center[1]) - (pi if pos_rel_center[1] < 0 else 0),
                    random.random() * 0.2
                ])
        # 1b. Next we create the while loop
        circles = [[(self.stdscr.getmaxyx()[0]//2, self.stdscr.getmaxyx()[1]//2), 0, 0]]
        winsize = self.stdscr.getmaxyx()
        max_radius = sqrt(winsize[0]**2 + winsize[1] ** 2) / 2
        time_ticks = 1
        running = True
        while(running):
            # If the gems are on the screen, or the last circle is still expanding, keep going
            # Note that here and later on we use the color == the clear color to identify the last circle
            running = not (circles[-1][2] == -1 and circles[-1][1] >= max_radius)
            # Erase
            self.stdscr.erase()
            # Draw circles
            for i in range(len(circles)):
                circle = circles[i]
                if circle[1] < max_radius:
                    circle[1] += max_radius * CIRCLE_GROWTH_SPEED
                if i == len(circles) - 1 or circle[1] > circles[i + 1][1]:
                    self.draw_circle(circle[0], floor(circle[1]), color=circle[2])

            if circles[-1][1] >= max_radius / 2 and circles[-1][2] != -1:
                circles.append([
                    (circles[-1][0][0], circles[-1][0][1]),
                    0,
                    circles[-1][2] + 1 if time_ticks < duration_ticks else -1
                ])
            # Otherwise, print the board, then do some cool physics to it.
            for row in range(self.board.HEIGHT):
                for col in range(self.board.WIDTH):
                    if positions[row][col][0] > GRAVITY:
                        # moves cursor (not game cursor)
                        ncurses_cursor = [
                            sin(ROTATION_SPEED * (1/positions[row][col][0]) * time_ticks + positions[row][col][1]) * positions[row][col][0] + winsize[0]/2,
                            cos(ROTATION_SPEED * (1/positions[row][col][0]) * time_ticks + positions[row][col][1]) * positions[row][col][0] + winsize[1]/2,
                        ]
                        positions[row][col][0] -= GRAVITY
                        # prints gems
                        gem_int = self.board.get_entry(row, col)
                        if gem_int in GEMS:
                            self.print(
                                GEMS[gem_int][0],
                                color=GEMS[gem_int][1],
                                end="",
                                coords=[floor(ncurses_cursor[0]), floor(ncurses_cursor[1])]
                            )
                    else:
                        self.board.map[row][col] = GEM_TYPES.blank

            # Done.
            time.sleep(0.05)
            time_ticks += 1
            self.stdscr.refresh()
        # Done.
        self.taking_input = True

    def get_gem_position(self, i, j) -> Vector:
        """
        Gets the position on the screen that a board piece is printed to.
        """
        return [i + floor((self.stdscr.getmaxyx()[0] / 2) - (self.board.HEIGHT / 2)) - 1,
                j*3 + floor((self.stdscr.getmaxyx()[1] / 2) - (self.board.WIDTH * 3/2)) + 1]


    # Board prints

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

    # Complex drawings -- uses calls to primitive draw functions

    def draw_circle(self, origin : tuple, radius : int, color=0):
        """
        Draws a circle with the midpoint circle algorithm, adapted from Wikipedia:
        https://en.wikipedia.org/wiki/Midpoint_circle_algorithm
        """
        winsize = self.stdscr.getmaxyx()
        t1 = radius / 16
        x = radius
        y = 0
        while(not (x < y)):
            # Draws points
            self.line(
                [origin[0] + y, origin[1] + x],
                [origin[0] - y, origin[1] + x],
                color=color
            )
            self.line(
                [origin[0] + y, origin[1] - x],
                [origin[0] - y, origin[1] - x],
                color=color
            )
            self.line(
                [origin[0] + x, origin[1] + y],
                [origin[0] - x, origin[1] + y],
                color=color
            )
            self.line(
                [origin[0] + x, origin[1] - y],
                [origin[0] - x, origin[1] - y],
                color=color
            )
            # Increments
            y = y + 1
            t1 = t1 + y
            t2 = t1 - x
            if t2 >= 0:
                t1 = t2
                x -= 1

    # Other

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

            case "warp":
                self.animation_warp()

            case "idkfa":
                self.board.score = 2**32

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
