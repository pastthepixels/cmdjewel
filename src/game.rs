// Handles game logic.

// Specifies how much points you get for each gem successfully swapped.
const POINTS_SWAP: u8 = 30;

// Specifies how much points you have to acquire before you level up.
const POINTS_LEVEL: u32 = 2000;

/// Types of gems to use.
enum Gems {
    Empty,
    Blue,
    White,
    Red,
    Yellow,
    Green,
    Orange,
    Purple,
}

/// Specifies a 2D x,y point
struct Point(u8, u8);

/// Specifies adjacent directions
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// cmdjewel boards.
pub struct Board {
    // All boards in Bejeweled (and hence, cmdjewel) are 8x8.
    data: [u8; 64],
    // Location of the cursor as a tuple.
    cursor: Point,
}

impl Board {
    fn new() -> Self {
        Board {
            data: [0; 64],
            cursor: Point(0, 0),
        }
    }

    /// Returns the current level as an integer.
    pub fn get_level(&self) -> u8 {
        todo!()
    }

    /// Returns the progress of the current level as an integer.
    /// $0 <= p <= 1$ for any progress $p$.
    pub fn get_level_progress(&self) {
        todo!()
    }

    /// Adds gems to the top row such that, if the gems fell, the lowest non-filled row is filled.
    pub fn fill_from_top(&mut self) {
        todo!()
    }

    /// Updates all physics by one frame.
    /// Follows the algorithm:
    /// - Start on the bottom-most row.
    /// 1. For all gems in the row, check if the space below them is empty.
    ///     - If it is empty, swap the gem with the space below it.
    ///     - Repeat step 2 for each gem in the row and remember that we found an empty space.
    /// 2. If no spaces in the row were empty, move up one row and then repeat step 1.
    pub fn update_physics_frame(&mut self) {
        todo!()
    }

    /// Swaps a gem with a gem in an adjacent direction, which points from the destination from the cursor. **Wrapper for private Board.swap.**
    pub fn swap(&mut self, direction: Direction) {
        todo!()
    }

    /// Swaps a gem with any other gem. `source` and `destination` are 2d coordinates.
    pub fn swap(&mut self, source: Point, destination: Point) {
        todo!()
    }

    /// Returns true if a point [x,y] is in a map.
    pub fn is_in_map(&self, point: Point) {
        todo!()
    }

    /// Finds all gems that match and returns their positions in the board.
    /// This can be used as a "dry run" to highlight any gems that have been matched.
    pub fn get_matching_gems(&self) -> Vec<Point> {
        todo!()
    }

    /// Finds all matching gems, and then:
    /// - Removes them, replacing them with empty spaces.
    /// - Adds points for each matching gem.
    /// - Adds special gems if applicable.
    pub fn update_matching_gems(&self) {
        todo!()
    }

    /// Returns true if you can make a move on the board.
    pub fn is_valid(&self) -> bool {
        todo!()
    }

    /// Returns true if you can make a move on a spot.
    pub fn is_valid_gem(&self, point: Point) -> bool {
        todo!()
    }

    /// Returns true if you can swap a gem, given the gem and direction of swappage.
    pub fn is_valid_move(&self, point: Point, direction: Direction) -> bool {
        todo!()
    }

    /// Moves the cursor by 1 in an adjacent direction to it.
    pub fn move_cursor(&self, direction: Direction) {
        todo!()
    }

    /// Gets the coordinates of the cursor
    pub fn get_cursor(&self) -> Point {
        todo!()
    }
}
