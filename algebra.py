from math import sqrt, floor

# 2D vector
class Vector2:
    def __init__(self, y=0, x=0):
        """
        Constructor with float data types.
        """
        self.x = x
        self.y = y

    def __floor__(self):
        return Vector2(x=floor(self.x), y=floor(self.y))

    def __getitem__(self, item):
        match item:
            case 0:
                return self.y
            case 1:
                return self.x

    def __setitem__(self, key, value):
        match key:
            case 0:
                self.y = value
            case 1:
                self.x = value

    def fromList(l: list):
        return Vector2(y=l[0], x=l[1])

    def dotProduct(self, other):
        return self.x * other.x + self.y * other.y

    def distanceTo(self, other) -> int:
        return sqrt((self.x - other.x)**2 + (self.y - other.y)**2)

    def length(self):
        return sqrt(self.x**2 + self.y**2)

    def __eq__(self, o):
        return self.x == o.x and self.y == o.y

    def __lt__(self, o):
        return self.x < o.x and self.y < o.y

    def __repr__(self):
        return f"[{self.y}, {self.x}]"

    def __add__(self, o):
        return Vector2(x=self.x + o.x, y=self.y + o.y)

    def __sub__(self, o):
        return Vector2(x=self.x - o.x, y=self.y - o.y)

    def __mul__(self, o):
        """
        Scalar multiplication
        """
        return Vector2(x=self.x * o, y=self.y * o)
