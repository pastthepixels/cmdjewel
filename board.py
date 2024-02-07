import curses
import os
import random
import traceback
import time
from enum import Enum, IntEnum
from math import floor
from console import StatusBar, ProgressBar

# Note: Negative numbers are reserved for inverted prints.
# ID's for each gem type - enum
GEM_TYPES = IntEnum("GEM_TYPES", ["blank", "diamond", "circle", "square", "kite", "green", "hexagon", "triangle"])
GEMS = { # Can remove
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

KEYBINDS = { # Can remove
    "left": [curses.KEY_LEFT, ord("h")],
    "right": [curses.KEY_RIGHT, ord("l")],
    "up": [curses.KEY_UP, ord("k")],
    "down": [curses.KEY_DOWN, ord("j")],
    "select": [curses.KEY_ENTER, ord(" ")]
}


class Board:
    """
    Board: Game logic for a given board, including Classic and Zen modes.
    """
    # board vars
    WIDTH = 8
    HEIGHT = 8
    running = True # <- can remove
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

    def __init__(self, game):
        self.map = []
        for i in range(self.HEIGHT):
            self.map.append([GEM_TYPES.blank for i in range(self.WIDTH)])
        self.game = game

    def get_entry(self, row, col):
        return self.map[row][col]

    def update(self):
        """
        Main game loop.
        """
        # TODO: cool animation as we scramble pieces
        #if self.score >= self.SCORE_PER_LEVEL and self.mode == self.modes.SELECT:
        #    self.level += 1
        self.update_board()

    def get_level(self):
        return self.score // self.SCORE_PER_LEVEL

    def get_level_progress(self):
        return self.score % self.SCORE_PER_LEVEL / self.SCORE_PER_LEVEL

    def swap(self, direction: str, cursor: list):
        """
        Swaps two points in the map -- via an i,jth point and a point defined by a movement of 1 from that point.
        :param direction: ["U", "D", "L", "R"]
        :param cursor: A list consisting of i, j coordinates for the "pivot" point
        :return:
        """
        cursor_swap = cursor.copy()
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
                (self.is_valid_move(cursor, cursor_swap) or self.is_valid_move(cursor_swap, cursor)):
            tmp = self.map[cursor[0]][cursor[1]]
            self.map[cursor[0]][cursor[1]
                                     ] = self.map[cursor_swap[0]][cursor_swap[1]]
            self.map[cursor_swap[0]][cursor_swap[1]] = tmp
            return True
        else:
            return False

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
        # Step 1: fill in the whole board, and make each gem fall to the bottom
        last_fall = True # Whether or not the last call to fall_all_and_replace resulted in something falling
        while last_fall == True:
            last_fall = self.fall_all_and_replace()
        # Step 2: check for matches.
        flagged_for_deletion = []  # array consisting of [row, col].
        # Flag things for deletion rather than deleting them right away. This also lets us check for chains.
        for row in range(self.HEIGHT):
            for col in range(self.WIDTH):
                # If top and bottom are the equal (vertical row)
                if self.is_in_map([row - 1, col]) and self.is_in_map([row + 1, col]) and\
                   self.map[row - 1][col] == self.map[row + 1][col] == self.map[row][col] and self.map[row][col] != GEM_TYPES.blank:
                    flagged_for_deletion.append([row - 1, col])
                    flagged_for_deletion.append([row, col])
                    flagged_for_deletion.append([row + 1, col])
                # If left and right are equal (horiz. row)
                if self.is_in_map([row, col - 1]) and self.is_in_map([row, col + 1]) and\
                   self.map[row][col - 1] == self.map[row][col + 1] == self.map[row][col] and self.map[row][col] != GEM_TYPES.blank:
                    flagged_for_deletion.append([row, col - 1])
                    flagged_for_deletion.append([row, col])
                    flagged_for_deletion.append([row, col + 1])
        # Add to the score.
        score = len(flagged_for_deletion) * self.SCORE_MULTIPLIER
        self.score += score
        # Turn everything we flagged into a blank space int
        if len(flagged_for_deletion) > 0:
            # Beep to indicate a match.
            curses.beep()
            # Make everything negative. This flags that we want to print it inverted. TODO
            for flag in flagged_for_deletion:
                if self.map[flag[0]][flag[1]] > 0:
                    self.map[flag[0]][flag[1]] *= -1
            self.game.update_all_windows()
            time.sleep(self.WAIT_TIME_ACTION)
        for flag in flagged_for_deletion:
            self.map[flag[0]][flag[1]] = GEM_TYPES.blank
        # We're done! Time to make your next move...
        self.allow_input = True
    
    def fall_all_and_replace(self) -> bool:
        """
        Makes every gem fall, and adds in new ones from the top if there are empty spaces.
        """
        fell = False
        # If a gem detects a space below it, it will keep swapping with that space until it doesn't detect one anymore.
        row = self.HEIGHT - 1
        while row >= 0:
            gem_fell = False
            for col in range(self.WIDTH):
                # note: don't fall blank spaces
                if self.map[row][col] != GEM_TYPES.blank:
                    if self.fall_gem(row, col) == True: gem_fell = True
            if gem_fell: # Reprint/wait only if a gem actually fell
                fell = True
                row += 1
                self.game.update_all_windows()
                time.sleep(self.WAIT_TIME_FALL)
            else:
                # Important for the loop to converge
                row -= 1
        # Now we add in more gems from the top (row == 0)
        top_empty = True
        while top_empty:
            top_empty = False
            for col in range(self.WIDTH):
                if self.map[0][col] == GEM_TYPES.blank:
                    top_empty = True
                    # Generate number between 0 and 6
                    self.map[0][col] = random.randint(GEM_TYPES.diamond, GEM_TYPES.triangle)
                    self.fall_gem(0, col)
        # Returns true if a gem fell
        return fell

    def get_game_state(self):
        """
        Returns true if the game is still in progress (moves can be made), else false 
        """
        # TODO: add a classic mode
        # If there isn't any more moves to make, refresh the board.
        exists_valid_piece = False
        for row in range(self.HEIGHT):
            for column in range(self.WIDTH):
                if self.is_valid_piece(row, column):
                    exists_valid_piece = True
        return exists_valid_piece

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

    def fall_gem(self, row, col) -> bool:
        """
        Makes a gem "fall" (go down each empty space, or blank)
        Assumes there is a VALID gem at row, col
        """
        if row + 1 < self.HEIGHT and self.map[row + 1][col] == GEM_TYPES.blank:
            self.map[row + 1][col] = self.map[row][col]
            self.map[row][col] = GEM_TYPES.blank
            return True
        else:
            return False

    def get_status(self) -> tuple:
        return (str(self.score), f"Level {self.get_level() + 1}", self.mode.name if self.mode else " ")
