// Handles game logic.
// TODO: remove the below line once the implementation is complete.
#![allow(dead_code, unused_variables)]

// Specifies how much points you get for each gem successfully swapped.
const POINTS_SWAP: u8 = 30;

// Specifies how much points you have to acquire before you level up.
const POINTS_LEVEL: u32 = 2000;

/// Types of gems to use.
#[derive(Copy, Clone)]
pub enum Gems {
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
#[derive(Copy, Clone)]
pub struct Point(usize, usize);

impl std::ops::Add<Point> for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

/// Specifies adjacent directions
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

/// cmdjewel boards.
pub struct Board {
    // All boards in Bejeweled (and hence, cmdjewel) are 8x8.
    // Whatever this is resized to, it MUST be a valid power of two.
    data: [Gems; 64],
    // Location of the cursor as a tuple.
    cursor: Point,
    // Current score
    score: u32,
}

impl Board {
    pub fn new() -> Self {
        Board {
            data: [Gems::Empty; 64],
            cursor: Point(0, 0),
            score: 0,
        }
    }

    /// Returns the current level as an integer.
    pub fn get_level(&self) -> u8 {
        (self.score / POINTS_LEVEL) as u8
    }

    /// Returns the progress of the current level as a floating point.
    /// $0 <= p <= 1$ for any progress $p$.
    pub fn get_level_progress(&self) -> f32 {
        (self.score % POINTS_LEVEL) as f32 / POINTS_LEVEL as f32
    }

    /// Adds gems to the top row such that, if the gems fell, the lowest non-filled row is filled.
    pub fn fill_from_top(&mut self) {
        // 1. Find the first row from the bottom that has at least 1 empty spot.
        let mut row_idx = self.get_width();
        let mut row_state = false;
        while row_idx > 0 && row_state != true {
            row_idx -= 1;
            for i in 0..self.get_width() {
                if let Gems::Empty = self.data[row_idx * self.get_width() + i] {
                    row_state = true;
                }
            }
        }
        // 2. Loop through that row, and find what slots are empty.
        if row_state {
            for i in 0..self.get_width() {
                // 3. Add corresponding gems, to the *top*, but only if gems don't already exist there.
                if let Gems::Empty = self.data[row_idx * self.get_width() + i] {
                    if let Gems::Empty = self.data[i] {
                        self.data[i] = Gems::Blue; // TODO: random generation, also nested ifs; good idea?
                    }
                }
            }
        }
    }

    /// Updates all physics by one frame.
    /// Follows the algorithm:
    /// - Start on the bottom-most row.
    /// 1. For all gems in the row, check if the space below them is empty.
    ///     - If it is empty, swap the gem with the space below it.
    ///     - Repeat step 2 for each gem in the row and remember that we found an empty space.
    /// 2. If no spaces in the row were empty, move up one row and then repeat step 1.
    pub fn update_physics_frame(&mut self) {
        for i in (0..self.data.len()).rev() {
            if i + self.get_width() < self.data.len() {
                if let Gems::Empty = self.data[i + self.get_width()] {
                    self.data[i + self.get_width()] = self.data[i];
                    self.data[i] = Gems::Empty;
                }
            }
        }
    }

    /// Swaps a gem with a gem in an adjacent direction, which points from the destination from the cursor. **Wrapper for private Board.swap.**
    pub fn swap(&mut self, direction: Direction) {
        let destination;
        match direction {
            Direction::Left => destination = self.cursor + Point(1, 0),
            Direction::Right => destination = self.cursor - Point(1, 0),
            Direction::Up => destination = self.cursor - Point(0, 1),
            Direction::Down => destination = self.cursor + Point(0, 1),
        }
        self.swap_explicit(self.cursor.clone(), destination)
    }

    /// Gets the width (=height) of the board.
    pub fn get_width(&self) -> usize {
        f32::sqrt(self.data.len() as f32) as usize
    }

    /// Swaps a gem with any other gem. `source` and `destination` are 2d coordinates.
    pub fn swap_explicit(&mut self, source: Point, destination: Point) {
        let destination_index = destination.1 * self.get_width() + destination.0;
        let source_index = source.1 * self.get_width() + source.0;
        // Stores a to be swapped value in memory.
        let temp = self.data[destination_index];
        // Swaps two things
        self.data[destination_index] = self.data[source_index];
        self.data[source_index] = temp;
    }

    /// Returns true if a point [x,y] is in a map.
    pub fn is_in_map(&self, point: Point) -> bool {
        (point.1 * self.get_width() + point.0) < self.data.len()
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
        // 1. If we swapped the piece, would we swap it outside of the board? Check each direction to make sure you even *can* swap the piece.
        todo!()
    }

    /// Returns true if you can swap a gem, given the gem and direction of swappage.
    pub fn is_valid_move(&self, point: Point, direction: Direction) -> bool {
        // 1. Copy the board, but swap the pieces in here. Check if the gem has:
        //    - Two pieces to its left/right
        //    - Two pieces above it/below it
        //    - One piece on either side horizontally/vertically
        todo!()
    }

    /// Moves the cursor by 1 in an adjacent direction to it.
    pub fn move_cursor(&mut self, direction: Direction) {
        let destination;
        match direction {
            Direction::Left => destination = self.cursor + Point(1, 0),
            Direction::Right => destination = self.cursor - Point(1, 0),
            Direction::Up => destination = self.cursor - Point(0, 1),
            Direction::Down => destination = self.cursor + Point(0, 1),
        }
        if self.is_in_map(destination) {
            self.cursor = destination;
        }
    }

    /// Gets the coordinates of the cursor
    pub fn get_cursor(&self) -> Point {
        self.cursor.clone()
    }

    /// Returns a reference to self.data
    pub fn as_ref(&self) -> &[Gems] {
        self.data.as_ref()
    }
}
