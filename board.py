import curses
import os
import random
import traceback
import time
from enum import Enum
from math import floor
from console import StatusBar, ProgressBar

# Note: Negative numbers are reserved for inverted prints.
GEMS = {
    # Blank space listed first
    -1: [" ", 1],
    # Gems!
    0: ["▼", 5],
    1: ["●", 0],
    2: ["■", 2],
    3: ["◆", 4],
    4: ["◎", 3],
    5: ["⬢", 10],
    6: ["▲", 148]
}

KEYBINDS = {
    "left": [curses.KEY_LEFT, ord("h")],
    "right": [curses.KEY_RIGHT, ord("l")],
    "up": [curses.KEY_UP, ord("k")],
    "down": [curses.KEY_DOWN, ord("j")],
    "select": [curses.KEY_ENTER, ord(" ")]
}


class Board:
    # board vars
    WIDTH = 8
    HEIGHT = 8
    running = True
    map = []

    # game vars
    SCORE_MULTIPLIER = 30  # how many points you get per gem swapped
    # TODO: increase this by a certain amount each level (maybe x1.5?), just like the actual game
    SCORE_PER_LEVEL = 2000
    modes = Enum("Modes", ["SELECT", "SWAP", "COMMAND"])
    mode = None
    cursor = [0, 0]
    allow_input = True
    score = 0  # This is just for a level
    total_score = 0  # This is the score across a game
    level = 0

    # statuses
    time_ticks = 0

    # Flashy visual things
    WAIT_TIME_ACTION = 0.5
    WAIT_TIME_FALL = 0.05

    # curses vars
    level_bar = None
    status_bar = None
    stdscr = None

    def __init__(self):
        self.map = []
        for i in range(self.HEIGHT):
            self.map.append([-1 for i in range(self.WIDTH)])

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
        # No cursor. Let us draw it.
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
        self.mode = self.modes.SELECT
        # Enter game loop:
        #   the game runs as fast as possible and gets input (opposed to updating per input)
        while self.running:
            try:
                # Updates the game/main screen
                self.stdscr.erase()
                self.update()
                self.stdscr.refresh()
            except curses.error:
                pass
            except:
                self.running = False
                self.__exit__(None, None, None)
                print(traceback.format_exc())

    def __exit__(self, exc_type, exc_val, exc_tb):
        self.stdscr.keypad(False)
        curses.nocbreak()
        curses.echo()
        curses.endwin()

    def update_status(self):
        """
        Updates the status screen.
        """
        self.status_bar.set_status_left(" | ".join(self.get_status()[:-1]))
        self.status_bar.set_status_right(self.get_status()[-1])
        self.status_bar.update()

    def update_level_bar(self):
        """
        Updates the level progress bar.
        """
        self.level_bar.set_progress(self.score, self.SCORE_PER_LEVEL)
        self.level_bar.update()

    def update(self):
        """
        game loop
        :return:
        """
        # Prints everything
        self.print_board()
        self.update_status()
        self.update_level_bar()
        # Handles input
        if self.allow_input:
            self.handle_input()
        # Handles mechanics
        # TODO: cool animation as we scramble pieces
        if self.score >= self.SCORE_PER_LEVEL and self.mode == self.modes.SELECT:
            self.score = 0
            self.level += 1
        self.update_board()

    def print(self, *strings: str, end="\n", color=None, reverse=False):
        for string in strings:
            if color is not None:
                self.stdscr.addstr(str(
                    string) + end, curses.color_pair(color) + (curses.A_REVERSE if reverse else 0))
            else:
                self.stdscr.addstr(str(string) + end,
                                   curses.A_REVERSE if reverse else 0)

    def handle_input(self):
        key = self.stdscr.getch()
        if key == ord("q"):
            self.running = False
        match self.mode:
            case self.modes.SELECT:
                if key in KEYBINDS["right"] or key in KEYBINDS["left"]:
                    direction = 1 if key in KEYBINDS["right"] else -1
                    if self.is_in_map([self.cursor[0], self.cursor[1] + direction]):
                        self.cursor[1] += direction
                if key in KEYBINDS["up"] or key in KEYBINDS["down"]:
                    direction = 1 if key in KEYBINDS["down"] else -1
                    if self.is_in_map([self.cursor[0] + direction, self.cursor[1]]):
                        self.cursor[0] += direction
                # Space to get into select mode.
                if key in KEYBINDS["select"]:
                    self.mode = self.modes.SWAP
                    return

            case self.modes.SWAP:
                # Escape key returns you -- so does another space
                if key == 27 or key in KEYBINDS["select"]:
                    self.mode = self.modes.SELECT

                if key in KEYBINDS["right"] or key in KEYBINDS["left"]:
                    self.swap("R" if key in KEYBINDS["right"] else "L")
                    self.mode = self.modes.SELECT

                if key in KEYBINDS["up"] or key in KEYBINDS["down"]:
                    self.swap("U" if key in KEYBINDS["up"] else "D")
                    self.mode = self.modes.SELECT

    # TODO develop

    def swap(self, direction: str):
        """
        Swaps two points in the map
        :param direction: ["U", "D", "L", "R"]
        :return:
        """
        cursor_swap = self.cursor.copy()
        match direction:
            case "U":
                cursor_swap[0] -= 1

            case "D":
                cursor_swap[0] += 1

            case "L":
                cursor_swap[1] -= 1

            case "R":
                cursor_swap[1] += 1
        # IF the swapped cursor is in the map, and either gem you are swapping forms a match
        if self.is_in_map(cursor_swap) and\
                (self.is_valid_move(self.cursor, cursor_swap) or self.is_valid_move(cursor_swap, self.cursor)):
            tmp = self.map[self.cursor[0]][self.cursor[1]]
            self.map[self.cursor[0]][self.cursor[1]
                                     ] = self.map[cursor_swap[0]][cursor_swap[1]]
            self.map[cursor_swap[0]][cursor_swap[1]] = tmp

    def is_in_map(self, pointyx: list):
        """
        Returns true if a [y, x] point is in the map.
        :param pointyx:
        :return:
        """
        return 0 <= pointyx[0] < self.HEIGHT and 0 <= pointyx[1] < self.WIDTH

    def update_board(self):
        """
        Checks for matches, then makes everything fall, and then restocks gems
        """
        self.allow_input = False  # Disable input until everything is done -- especially animations.
        flagged_for_deletion = []  # array consisting of [row, col].
        # Flag things for deletion rather than deleting them right away. This also lets us check for chains.
        for row in range(self.HEIGHT):
            for col in range(self.WIDTH):
                # If top and bottom are the equal (vertical row)
                if self.is_in_map([row - 1, col]) and self.is_in_map([row + 1, col]) and\
                   self.map[row - 1][col] == self.map[row + 1][col] == self.map[row][col] and self.map[row][col] != -1:
                    flagged_for_deletion.append([row - 1, col])
                    flagged_for_deletion.append([row, col])
                    flagged_for_deletion.append([row + 1, col])
                # If left and right are equal (horiz. row)
                if self.is_in_map([row, col - 1]) and self.is_in_map([row, col + 1]) and\
                   self.map[row][col - 1] == self.map[row][col + 1] == self.map[row][col] and self.map[row][col] != -1:
                    flagged_for_deletion.append([row, col - 1])
                    flagged_for_deletion.append([row, col])
                    flagged_for_deletion.append([row, col + 1])
        # Add to the score.
        score = len(flagged_for_deletion) * self.SCORE_MULTIPLIER
        self.total_score += score
        self.score += score
        # Turn everything we flagged into a blank space int
        if len(flagged_for_deletion) > 0:
            # Make everything negative. This flags that we want to print it inverted.
            for flag in flagged_for_deletion:
                if self.map[flag[0]][flag[1]] > -10:
                    self.map[flag[0]][flag[1]] *= -1
                    self.map[flag[0]][flag[1]] -= 10
            self.reprint_board()
            time.sleep(self.WAIT_TIME_ACTION)
        for flag in flagged_for_deletion:
            self.map[flag[0]][flag[1]] = -1
        # We flagged what we want to remove by making it an empty space (-1). Now we want to make gems fall!
        self.fall_all_and_replace()
        # Check for an end state -- and do something about that.
        self.check_end_state()
        # We're done! Time to make your next move...
        self.allow_input = True
    
    def fall_all_and_replace(self):
        """
        Makes every gem fall, and adds in new ones from the top if there are empty spaces.
        """
        # If a gem detects a space below it, it will keep swapping with that space until it doesn't detect one anymore.
        for row in range(self.HEIGHT):
            for col in range(self.WIDTH):
                # note: don't fall -1's
                if self.map[row][col] != -1:
                    self.fall_gem(row, col)
        # Now we add in more gems from the top (row == 0)
        top_empty = True
        while top_empty:
            top_empty = False
            for col in range(self.WIDTH):
                if self.map[0][col] == -1:
                    top_empty = True
                    # Generate number between 0 and 6
                    self.map[0][col] = random.randint(0, 6)
                    self.fall_gem(0, col)

    def check_end_state(self):
        """
        Checks if any moves can be made, and if not, regenerates the board.
        """
        # TODO: add a classic mode
        # If there isn't any more moves to make, refresh the board.
        exists_valid_piece = False
        for row in range(self.HEIGHT):
            for column in range(self.WIDTH):
                if self.is_valid_piece(row, column):
                    exists_valid_piece = True
        if not exists_valid_piece:
            self.map = []
            for i in range(self.HEIGHT):
                self.map.append([-1 for i in range(self.WIDTH)])

    def is_valid_piece(self, row, col) -> bool:
        """
        Checks if a piece has any valid moves.
        """
        gem = self.map[row][col]
        # First checks if any of the pieces around the shape are the same as the shape.
        adjacent = [
            [row - 1, col + 1],
            [row, col + 1],
            [row + 1, col + 1],
            [row + 1, col],
            [row + 1, col - 1],
            [row, col - 1],
            [row - 1, col - 1],
            [row - 1, col]
        ]
        for coords in adjacent:
            # Just find *an* adjacent piece (can be the first one). That's all we care about.
            if self.is_in_map(coords) and self.map[coords[0]][coords[1]] == gem:
                # Try swapping the gem in all directions and seeing if it is valid
                return (self.is_in_map([row - 1, col]) and self.is_valid_move([row, col], [row - 1, col])) or\
                    (self.is_in_map([row + 1, col]) and self.is_valid_move([row, col], [row + 1, col])) or\
                    (self.is_in_map([row, col - 1]) and self.is_valid_move([row, col], [row, col - 1])) or\
                    (self.is_in_map(
                        [row, col + 1]) and self.is_valid_move([row, col], [row, col + 1]))
        return False

    def is_valid_move(self, original_coords: list, swapped_coords: list):
        """
        Assumes `shape` is at row, col and checks to see if it matches anything.
        Instead of the check in update_board(), this has to check if the piece is an edge piece!
        """
        gem = self.map[original_coords[0]][original_coords[1]]
        # HORIZONTAL CHAINS:
        # Get the whole row
        row = self.map[swapped_coords[0]].copy()
        # Swap what we need
        if original_coords[0] == swapped_coords[0]:
            row[original_coords[1]] = row[swapped_coords[1]]
        row[swapped_coords[1]] = gem
        # Run through the whole thing, count the chain
        if self.get_longest_chain_in_row(row, gem) >= 3:
            return True
        # VERTICAL CHAINS:
        # Get the whole column (harder)
        column = [self.map[i][swapped_coords[1]] for i in range(self.HEIGHT)]
        # Swap what we need
        if original_coords[1] == swapped_coords[1]:
            column[original_coords[0]] = column[swapped_coords[0]]
        column[swapped_coords[0]] = gem
        return self.get_longest_chain_in_row(column, gem) >= 3

    def get_longest_chain_in_row(self, row: list, gem):
        chain_active = False
        longest_chain = 0
        chain = 0
        for g in row:
            if g == gem:
                chain += 1
                chain_active = True
            elif g != gem and chain_active:
                chain_active = False
                if chain > longest_chain:
                    longest_chain = chain
                chain = 0
        if chain > longest_chain:
            longest_chain = chain
        return longest_chain

    def fall_gem(self, row, col):
        """
        Makes a gem "fall" (go down each empty space, or -1)
        Assumes there is a VALID gem at row, col
        """
        space_detected = False
        nrow = row  # n stands for new!
        ncol = col
        wait_time = self.WAIT_TIME_FALL
        while self.is_in_map([nrow + 1, ncol]) and not space_detected:
            if self.map[nrow + 1][ncol] == -1:
                self.reprint_board()
                time.sleep(wait_time)
                # Acceleration! It helps make things feel less slow.
                wait_time *= 0.3
                self.map[nrow + 1][ncol] = self.map[nrow][ncol]
                self.map[nrow][ncol] = -1
                nrow += 1
            else:
                space_detected = True

    def reprint_board(self):
        self.stdscr.erase()
        self.print_board()
        self.update_status()
        self.update_level_bar()
        self.stdscr.refresh()

    def print_board(self):
        """
        Prints the board out. You need a separate function for printing the status.
        :return:
        """
        # Prints a margin to vertical center the grid.
        self.print("\n" * floor((self.stdscr.getmaxyx()
                   [0] / 2) - (self.HEIGHT / 2)), end="")
        for row in range(self.HEIGHT):
            # Prints a margin before each row to horizontal center the grid (SELF.WIDTH * 3 == 3 characters per column; space, char, space
            self.print(
                " " * (floor((self.stdscr.getmaxyx()[1] / 2) - (self.WIDTH * 3/2))), end="")
            # Prints gems
            for col in range(self.WIDTH):
                inverted = False
                gem_int = self.map[row][col]
                # Checks to see if there is a negative flag saying the number is inverted...
                if gem_int <= -10:
                    inverted = True
                    gem_int += 10
                    gem_int /= -1
                # Pick a dynamic color per mode, or use the gem's color
                color = GEMS[gem_int][1] if gem_int in GEMS else None
                if row == self.cursor[0] and col == self.cursor[1] and self.allow_input:
                    match self.mode:
                        case self.modes.SELECT:
                            color = None

                        case self.modes.SWAP:
                            color = 2
                self.print(
                    " " + str(GEMS[gem_int][0]
                              if gem_int in GEMS else gem_int),
                    end=" ",
                    color=color,
                    reverse=(
                        row == self.cursor[0] and col == self.cursor[1] and self.allow_input) or inverted
                )
            self.print("")

    def get_status(self) -> tuple:
        return (str(self.total_score), f"Level {self.level + 1}", self.mode.name if self.mode else " ")
