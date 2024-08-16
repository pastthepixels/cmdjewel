// Handles game logic.
// TODO: remove the below line once the implementation is complete.
#![allow(dead_code, unused_variables)]

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// Specifies how much points you get for each gem successfully swapped.
const POINTS_SWAP: u8 = 30;

// Specifies how much points you have to acquire before you level up.
const POINTS_LEVEL: u32 = 2000;

/// Types of gems to use.
#[derive(Copy, Clone, PartialEq, Eq)]
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

impl Distribution<Gems> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gems {
        match rng.gen_range(0..=6) {
            0 => Gems::Blue,
            1 => Gems::White,
            2 => Gems::Red,
            3 => Gems::Yellow,
            4 => Gems::Green,
            5 => Gems::Orange,
            _ => Gems::Purple,
        }
    }
}

/// Specifies a 2D x,y point
#[derive(Copy, Clone)]
pub struct Point(pub usize, pub usize);

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
#[derive(Copy, Clone, PartialEq, Eq)]
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

    /// Gets the score
    pub fn get_score(&self) -> u32 {
        self.score
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
                        self.data[i] = rand::random(); // TODO: nested ifs; good idea?
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
        let destination = match direction {
            Direction::Left => self.cursor - Point(1, 0),
            Direction::Right => self.cursor + Point(1, 0),
            Direction::Up => self.cursor - Point(0, 1),
            Direction::Down => self.cursor + Point(0, 1),
        };
        self.swap_explicit(self.cursor.clone(), destination)
    }

    /// Gets the width (=height) of the board.
    pub fn get_width(&self) -> usize {
        f32::sqrt(self.data.len() as f32) as usize
    }

    /// Swaps a gem with any other gem. `source` and `destination` are 2d coordinates.
    pub fn swap_explicit(&mut self, source: Point, destination: Point) {
        let destination_index = self.point_to_index(destination);
        let source_index = self.point_to_index(source);
        // Stores a to be swapped value in memory.
        let temp = self.data[destination_index];
        // Swaps two things
        self.data[destination_index] = self.data[source_index];
        self.data[source_index] = temp;
    }

    /// Returns true if a point [x,y] is in a the board.
    pub fn is_in_board(&self, point: Point) -> bool {
        point.1 < self.get_width() && point.0 < self.get_width()
    }

    /// Finds all gems that match and returns their positions in the board.
    /// This can be used as a "dry run" to highlight any gems that have been matched.
    pub fn get_matching_gems(&self) -> Vec<Point> {
        let mut valid_gems: Vec<Point> = Vec::new();
        for i in 0..self.data.len() {
            let point = self.index_to_point(i);
            if self.data[i] != Gems::Empty && self.is_matching_gem(self.data.as_ref(), point) {
                valid_gems.push(point);
            }
        }
        valid_gems
    }

    /// Finds all matching gems, and then:
    /// - Removes them, replacing them with empty spaces.
    /// - Adds points for each matching gem.
    /// - Adds special gems if applicable.
    pub fn update_matching_gems(&mut self) {
        self.get_matching_gems().iter().for_each(|point| {
            self.data[self.point_to_index(*point)] = Gems::Empty;
            self.score += POINTS_SWAP as u32;
        })
    }

    /// Returns true if the entire board is filled with gems.
    pub fn is_full(&self) -> bool {
        self.data.iter().find(|x| Gems::Empty == **x) == None
    }

    /// Returns true if you can make a move on the board.
    pub fn is_valid(&self) -> bool {
        for i in 0..self.data.len() {
            let point = self.index_to_point(i);
            if self.data[i] != Gems::Empty && self.is_valid_gem(point) {
                return true;
            }
        }
        false
    }

    /// Returns true if you can make a move on a spot.
    pub fn is_valid_gem(&self, point: Point) -> bool {
        // 1. If we swapped the piece, would we swap it outside of the board? Check each direction to make sure you even *can* swap the piece.
        self.is_valid_move(point, Direction::Left)
            || self.is_valid_move(point, Direction::Right)
            || self.is_valid_move(point, Direction::Up)
            || self.is_valid_move(point, Direction::Down)
    }

    /// Returns true if you can swap a gem, given the gem and direction of swappage.
    pub fn is_valid_move(&self, point: Point, direction: Direction) -> bool {
        // Ensure that we aren't subtracting from a (0,0)
        if point.0 == 0 && direction == Direction::Left
            || point.1 == 0 && direction == Direction::Up
        {
            false
        } else {
            // Store the destination coordinates
            let destination = match direction {
                Direction::Left => point - Point(1, 0),
                Direction::Right => point + Point(1, 0),
                Direction::Up => point - Point(0, 1),
                Direction::Down => point + Point(0, 1),
            };
            // 1. Check if the cursor and destination are in the map.
            if self.is_in_board(self.cursor) && self.is_in_board(destination) {
                // 2. Copy the board
                let mut data_copy = self.data.clone();
                // 3. Swap the gems in this board.
                let destination_index = self.point_to_index(destination);
                let source_index = self.point_to_index(point);
                data_copy[destination_index] = self.data[source_index];
                data_copy[source_index] = self.data[destination_index];
                // We're swapping gems, so we have to check if either gem swapped produces a match.
                self.is_matching_gem(data_copy.as_ref(), destination)
                    || self.is_matching_gem(data_copy.as_ref(), point)
            } else {
                false
            }
        }
    }

    /// Returns true if a gem is matching.
    /// Check if the gem has:
    ///    - Two pieces to its left/right
    ///    - Two pieces above it/below it
    ///    - One piece on either side horizontally/vertically
    pub fn is_matching_gem(&self, data: &[Gems], point: Point) -> bool {
        let point_index = self.point_to_index(point);
        // Two pieces to the left
        if point.0 >= 2
            && data[self.point_to_index(point - Point(1, 0))] == data[point_index]
            && data[self.point_to_index(point - Point(2, 0))] == data[point_index]
        {
            true
        // Two pieces to the right
        } else if self.is_in_board(point + Point(2, 0))
            && data[self.point_to_index(point + Point(1, 0))] == data[point_index]
            && data[self.point_to_index(point + Point(2, 0))] == data[point_index]
        {
            true
        // Two pieces below it
        } else if self.is_in_board(point + Point(0, 2))
            && data[self.point_to_index(point + Point(0, 1))] == data[point_index]
            && data[self.point_to_index(point + Point(0, 2))] == data[point_index]
        {
            true
        // Two pieces above it
        } else if point.1 >= 2
            && data[self.point_to_index(point - Point(0, 1))] == data[point_index]
            && data[self.point_to_index(point - Point(0, 2))] == data[point_index]
        {
            true
        // Horizontal middle
        } else if point.0 >= 1
            && self.is_in_board(point + Point(1, 0))
            && data[self.point_to_index(point - Point(1, 0))] == data[point_index]
            && data[self.point_to_index(point + Point(1, 0))] == data[point_index]
        {
            true
        // Vertical middle
        } else if point.1 >= 1
            && self.is_in_board(point - Point(0, 1))
            && self.is_in_board(point + Point(0, 1))
            && data[self.point_to_index(point - Point(0, 1))] == data[point_index]
            && data[self.point_to_index(point + Point(0, 1))] == data[point_index]
        {
            true
        } else {
            false
        }
    }

    /// Moves the cursor by 1 in an adjacent direction to it.
    pub fn move_cursor(&mut self, direction: Direction) {
        let destination = match direction {
            Direction::Left => self.cursor - Point(1, 0),
            Direction::Right => self.cursor + Point(1, 0),
            Direction::Up => self.cursor - Point(0, 1),
            Direction::Down => self.cursor + Point(0, 1),
        };
        if self.is_in_board(destination) {
            self.cursor = destination;
        }
    }

    /// Sets the cursor to a particular point.
    pub fn set_cursor(&mut self, point: Point) {
        self.cursor = point;
    }

    /// Gets the coordinates of the cursor
    pub fn get_cursor(&self) -> Point {
        self.cursor.clone()
    }

    /// Returns a reference to self.data
    pub fn as_ref(&self) -> &[Gems] {
        self.data.as_ref()
    }

    /// Converts a Point to an index in self.data.
    pub fn point_to_index(&self, point: Point) -> usize {
        point.1 * self.get_width() + point.0
    }

    /// Converts an index in self.data to a Point.
    pub fn index_to_point(&self, index: usize) -> Point {
        let row = index / self.get_width();
        Point(index - row * self.get_width(), row)
    }
}
