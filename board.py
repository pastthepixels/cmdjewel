import curses, os, random, traceback, time
from enum import Enum
from math import floor

# Note: Negative numbers are reserved for inverted prints.
GEMS = {
    # Blank space listed first
    -1: [" ", 1],
    # Gems!
    0: ["▼", 5],
    1: ["⬤", 0],
    2: ["■", 2],
    3: ["⯁", 4],
    4: ["◎", 3],
    5: ["⬢", 10],
    6: ["▲", 148]
}


class Board:
    # board vars
    WIDTH = 8
    HEIGHT = 8
    running = True
    map = []

    # game vars
    modes = Enum("Modes", ["SELECT", "SWAP", "COMMAND"])
    mode = None
    cursor = [0, 0]
    allow_input = True

    # statuses
    time_ticks = 0

    # Flashy visual things
    WAIT_TIME_ACTION = 0.5
    WAIT_TIME_FALL = 0.05

    # curses vars
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
        self.stdscr.resize(curses.LINES - len(self.get_status()) - 1, curses.COLS)
        self.status_bar = curses.newwin(len(self.get_status()) + 1, curses.COLS,
                                        curses.LINES - len(self.get_status()) - 1, 0)
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
        #for i in range(255):
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
                # Updates the status bar
                self.update_status()
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
        self.status_bar.erase()
        for i, status in enumerate(self.get_status()):
            self.status_bar.addstr(i + 1, 0, status, curses.A_REVERSE)
            self.status_bar.addstr(i + 1, len(status), " " * (curses.COLS - len(status) - 1), curses.A_REVERSE)
        self.status_bar.refresh()

    def update(self):
        """
        game loop
        :return:
        """
        self.print_board()
        if self.allow_input: self.handle_input()
        self.update_board()

    def print(self, *strings: str, end="\n", color=None, reverse=False):
        for string in strings:
            if color is not None:
                self.stdscr.addstr(str(string) + end, curses.color_pair(color) + (curses.A_REVERSE if reverse else 0))
            else:
                self.stdscr.addstr(str(string) + end, curses.A_REVERSE if reverse else 0)

    key = 0
    def handle_input(self):
        key = self.stdscr.getch()
        if key != -1: self.key = key
        if key == ord("q"):
            self.running = False
        match self.mode:
            case self.modes.SELECT:
                if key == curses.KEY_RIGHT or key == curses.KEY_LEFT:
                    direction = 1 if key == curses.KEY_RIGHT else -1
                    if self.is_in_map([self.cursor[0], self.cursor[1] + direction]):
                        self.cursor[1] += direction 
                if key == curses.KEY_UP or key == curses.KEY_DOWN:
                    direction = 1 if key == curses.KEY_DOWN else -1
                    if self.is_in_map([self.cursor[0] + direction, self.cursor[1]]):
                        self.cursor[0] += direction
                # Space to get into select mode.
                if key == ord(" "):
                    self.mode = self.modes.SWAP
                    return

            case self.modes.SWAP:
                # Escape key returns you -- so does another space
                if key == 27 or key == ord(" "):
                    self.mode = self.modes.SELECT

                if key == curses.KEY_RIGHT or key == curses.KEY_LEFT:
                    self.swap("R" if key == curses.KEY_RIGHT else "L")
                    self.mode = self.modes.SELECT

                if key == curses.KEY_UP or key == curses.KEY_DOWN:
                    self.swap("U" if key == curses.KEY_UP else "D")
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
            self.map[self.cursor[0]][self.cursor[1]] = self.map[cursor_swap[0]][cursor_swap[1]]
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
        self.allow_input = False # Disable input until everything is done -- especially animations.
        flagged_for_deletion = [] # array consisting of [row, col].
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
        # If a gem detects a space below it, it will keep swapping with that space until it doesn't detect one anymore.
        for row in range(self.HEIGHT):
            for col in range(self.WIDTH):
                # note: don't fall -1's
                if self.map[row][col] != -1: self.fall_gem(row, col)
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
        # TODO: add a classic mode
        # If there isn't any more moves to make, refresh the board.
        exists_valid_piece = False
        for row in range(self.HEIGHT):
            for column in range(self.WIDTH):
                if self.is_valid_piece(row, column):
                    exists_valid_piece = True
        if exists_valid_piece == False:
            self.map = []
            for i in range(self.HEIGHT):
                self.map.append([-1 for i in range(self.WIDTH)])
        # We're done! Time to make your next move...
        self.allow_input = True

    def is_valid_piece(self, row, col):
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
                        (self.is_in_map([row, col + 1]) and self.is_valid_move([row, col], [row, col + 1]))


    # TODO: refactor, this is some HORRIBLE spaghetti code
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
            elif g != gem and chain_active == True:
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
        nrow = row # n stands for new!
        ncol = col
        while self.is_in_map([nrow + 1, ncol]) and space_detected == False:
            if self.map[nrow + 1][ncol] == -1:
                self.reprint_board()
                time.sleep(self.WAIT_TIME_FALL)
                self.map[nrow + 1][ncol] = self.map[nrow][ncol]
                self.map[nrow][ncol] = -1
                nrow += 1
            else:
                space_detected = True

    def reprint_board(self):
        self.stdscr.erase()
        self.print_board()
        self.stdscr.refresh()


    def print_board(self):
        """
        Prints the board out. You need a separate function for printing the status.
        :return:
        """
        # Prints a margin to vertical center the grid.
        self.print("\n" * floor((self.stdscr.getmaxyx()[0] / 2) - (self.HEIGHT / 2)), end="")
        for row in range(self.HEIGHT):
            # Prints a margin before each row to horizontal center the grid (SELF.WIDTH * 3 == 3 characters per column; space, char, space
            self.print(" " * (floor((self.stdscr.getmaxyx()[1] / 2) - (self.WIDTH * 3/2))), end="")
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
                        " " + str(GEMS[gem_int][0] if gem_int in GEMS else gem_int),
                        end=" ",
                        color=color,
                        reverse=(row == self.cursor[0] and col == self.cursor[1] and self.allow_input) or inverted
                    )
            self.print("")
        # TODO get rid of this following line
        #self.print(str(self.key))
        # moves ncurses cursor to character
        last_cursor_pos = self.stdscr.getyx()

    def get_status(self) -> tuple:
        return (self.mode.name if self.mode else " ",)
