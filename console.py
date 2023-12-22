# ncurses stuff that should be encapsulated from board.py
from curses import color_pair, A_REVERSE

# Status bars (idea from https://piped.video/watch?v=ga0Kh97VHsI)
class StatusBar:
    """
    Draws a status bar on a parent screen.
    """
    def __init__(self, screen, y: int, color = 0):
        self.screen = screen
        self.color = color
        self.y = y # Denotes space from the bottom of the screen
        self.status = { # Creates self.status, an empty dictionary (see set_status for more)
            "left": "",
            "middle": "",
            "right": ""
        }
    
    def set_status(self, left="", middle="", right=""):
        """
        The variable storing the status is a dictionary that holds three values
        for different justifications of text (sorta like Waybar)
        """
        self.set_status_left(left)
        self.set_status_middle(middle)
        self.set_status_right(right)

    def set_status_left(self, text="", padding=1):
        self.status["left"] = text.center(len(text) + padding*2)

    def set_status_middle(self, text="", padding=1):
        self.status["middle"] = text.center(len(text) + padding*2)

    def set_status_right(self, text="", padding=1):
        self.status["right"] = text.center(len(text) + padding*2)

    def update(self):
        rows, cols = self.screen.getmaxyx()
        y = rows - self.y - 1
        # First, draw a horizontal line of the background color.
        self.screen.insstr(y, 0, " " * (cols), color_pair(self.color) + A_REVERSE)
        # Then, draw the left status.
        if self.status["left"]:
            self.screen.addstr(y, 0, self.status["left"], color_pair(self.color) + A_REVERSE)
        # Then, draw the middle status.
        if self.status["middle"]:
            self.screen.addstr(y, cols/2 - len(self.status["middle"]/2), self.status["middle"], color_pair(self.color) + A_REVERSE)
        # Lastly, draw the right status.
        if self.status["right"]:
            self.screen.insstr(y, cols - len(self.status["right"]), self.status["right"], color_pair(self.color) + A_REVERSE)

class ProgressBar:
    def __init__(self, screen, progress: float, foreground_color: int, y: int, max_progress = 1, background_color: int = None):
        self.screen = screen
        self.progress = progress
        self.y = y
        self.max_progress = max_progress
        self.foreground_color = foreground_color
        self.background_color = background_color

    def set_progress(self, progress: float, max_progress = 1):
        self.progress = progress
        self.max_progress = max_progress

    def update(self):
        rows, cols = self.screen.getmaxyx()
        y = rows - self.y - 1
        if self.background_color:
            self.screen.addstr(y, 0, " " * cols, color_pair(self.background_color) + A_REVERSE)
        if cols > (self.progress / self.max_progress) > 0:
            self.screen.addstr(y, 0, " " * int(self.progress / self.max_progress * cols), color_pair(self.foreground_color) + A_REVERSE)
