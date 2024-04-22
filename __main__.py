from game import Game
from importlib import import_module
import os

# Change directory to the place where the .py file is so everything is relative to that
os.chdir(os.path.dirname(__file__))

# See if Pygame is installed, and try to play music. (This isn't necessary!)
# This game uses beyond_the_network.it:
#   by Skaven252,
#   retrieved from [The Mod Archive](https://modarchive.org/index.php?request=view_by_moduleid&query=156184),
#   and under a CC-BY-NC-ND 4.0 license.
try:
    pygame = import_module('pygame')
    pygame.init()
    pygame.mixer.music.load("beyond_the_network.it")
    pygame.mixer.music.play()
except:
    print("PyGame or libopenmpt not found. To play music, install PyGame with pip.")
    print("On some systems, you might have to also install OpenMPT -- specifically libmodplug.")

# Play cmdjewel!
with Game() as game:
    pass
