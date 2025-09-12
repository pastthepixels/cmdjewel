// Handles game logic.

use rand::seq::SliceRandom;

use crate::point::{Direction, Point};

use crate::constants::*;

use crate::gems::*;

/// Configuring cmdjewel boards (for different gamemodes)
pub enum Gamemode {
    CLASSIC,
    ZEN
}

pub struct BoardConfig {
    pub infinite: bool,
    pub name: String,
    pub gamemode: Gamemode
}

impl BoardConfig {
    pub fn new_classic() -> Self {
        BoardConfig {
            infinite: false,
            name: "classic".into(),
            gamemode: Gamemode::CLASSIC
        }
    }

    pub fn new_zen() -> Self {
        BoardConfig {
            infinite: true,
            name: "zen".into(),
            gamemode: Gamemode::ZEN
        }
    }
}

/// cmdjewel boards.
pub struct Board {
    // All boards in Bejeweled (and hence, cmdjewel) are 8x8.
    // Whatever this is resized to, it MUST be a valid power of two. (or else we get runtime errors ☹)
    data: [Gem; 64],
    buffer: [Gem; 64],
    // Location of the cursor as a tuple.
    cursor: Point<usize>,
    // Current score/level
    score: u32,
    level: u8,
    level_progress: f32,
    // Config
    config: BoardConfig,
}

impl Board {
    pub fn new(config: BoardConfig) -> Self {
        Board {
            data: [Gem::Empty; 64],
            buffer: [Gem::Empty; 64],
            cursor: Point(0, 0),
            score: 0,
            level: 0,
            level_progress: 0.0,
            config,
        }
    }

    pub fn new_controlled(config: BoardConfig, data: [Gem; 64], score: u32, level: u8, level_progress: f32) -> Self {
        Board {
            data,
            buffer: [Gem::Empty; 64],
            cursor: Point(0, 0),
            score,
            level,
            level_progress,
            config,
        }
    }

    fn from_data(data: [Gem; 64]) -> Self {
        Board {
            data,
            buffer: [Gem::Empty; 64],
            cursor: Point(0, 0),
            score: 0,
            level: 0,
            level_progress: 0.0,
            config: BoardConfig::new_classic(),
        }
    }

    /// Returns the current level as an integer.
    pub fn get_level(&self) -> u8 {
        self.level
    }

    /// Returns the progress of the current level as a floating point.
    /// $0 <= p <= 1$ for any progress $p$.
    pub fn get_level_progress(&self) -> f32 {
        self.level_progress.min(1.0)
    }

    /// Every time a gem is swapped, points are added, and the gem contributes some amount to making progress though a level.
    /// This function returns that amount based on the current level.
    pub fn get_swap_progress(&self) -> f32 {
        (PROGRESS_SWAP_INITIAL * PROGRESS_SWAP_FALLOFF.powi(self.level as i32))
            .max(PROGRESS_SWAP_MIN)
    }

    /// Increments the level by 1 and resets level_progress if level_progress is geq 1
    pub fn update_level(&mut self) {
        if self.level_progress >= 1.0 {
            self.level_progress -= 1.0;
            self.level += 1;
        }
    }

    /// Gets a gem from a point
    pub fn get_gem(&mut self, point: Point<usize>) -> Gem {
        self.data[self.point_to_index(point)]
    }

    /// Gets the score
    pub fn get_score(&self) -> u32 {
        self.score
    }

    /// Returns true if the buffer is empty (and can be filled).
    pub fn is_buffer_empty(&self) -> bool {
        self.buffer.iter().all(|&x| x == Gem::Empty)
    }

    /// Fills the buffer:
    ///        a. Clone data and make everything fall down.
    ///        b. Insert new gems until everything is valid (brute force)
    ///        c. Record the gems we inserted and their positions in the gem buffer.
    pub fn fill_gem_buffer(&mut self) {
        // 1. Clone data and make everything fall down.
        let mut data_clone = self.data;
        loop {
            let mut gem_fell = false;
            for i in 0..(data_clone.len() - self.get_width()) {
                if data_clone[i + self.get_width()] == Gem::Empty && data_clone[i] != Gem::Empty {
                    data_clone[i + self.get_width()] = data_clone[i];
                    data_clone[i] = Gem::Empty;
                    gem_fell = true;
                }
            }
            if !gem_fell {
                break;
            }
        }
        // 2. If it's supposed to be infinite, but it isn't valid, do some checks...
        // 2. Insert new gems until everything is valid (brute force)
        let mut iterations = 0;
        loop {
            // Consider some case starting from data_clone
            let case: [Gem; 64] = data_clone
                .into_iter()
                .map(|gem| {
                    if gem == Gem::Empty {
                        rand::random()
                    } else {
                        gem
                    }
                })
                .collect::<Vec<Gem>>()
                .try_into()
                .unwrap_or([Gem::Empty; 64]);

            // check if the case is valid
            if iterations > 500 {
                for i in 0..case.len() {
                    if case[i] != data_clone[i] {
                        self.buffer[i] = case[i];
                    }
                }
                // Find the first empty spot in data_clone, make that a hypercube
                for (i, gem) in data_clone.iter().enumerate() {
                    if *gem == Gem::Empty {
                        self.buffer[i] = Gem::Hypercube(GemSelector::None);
                    }
                }
                break;
            } else if !self.config.infinite || Board::from_data(case).is_valid() {
                // Record the gems we inserted and their positions in the gem buffer.
                for i in 0..case.len() {
                    if case[i] != data_clone[i] {
                        self.buffer[i] = case[i];
                    }
                }
                break;
            } else {
                iterations += 1;
            }
        }
    }

    /// Slides gems down by 1, and fill the topmost row with the lowest row from the buffer.
    pub fn slide_down(&mut self) {
        // Slides gems down by 1
        for i in (0..(self.data.len() - self.get_width())).rev() {
            if let Gem::Empty = self.data[i + self.get_width()] {
                self.data[i + self.get_width()] = self.data[i];
                self.data[i] = Gem::Empty;
            }
        }

        // Searches backward through the buffer until it finds the first non-empty space,
        // and then adds that (and continues adding non-empty spaces) until the beginning
        // of that row.
        let mut non_empty_found = false;
        for i in (0..self.buffer.len()).rev() {
            let x = self.index_to_point(i).0;
            if self.buffer[i] != Gem::Empty && self.data[x] == Gem::Empty {
                non_empty_found = true;
                self.data[x] = self.buffer[i];
                self.buffer[i] = Gem::Empty;
            }
            if non_empty_found && i % self.get_width() == 0 {
                break;
            }
        }
    }

    /// Shuffles the board (until we have a valid board).
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        while !self.is_valid() {
            self.data.shuffle(&mut rng);
        }
    }

    /// Swaps a gem with a gem in an adjacent direction, which points from the destination from the cursor. **Wrapper for private Board.swap.**
    pub fn swap(&mut self, direction: Direction) {
        // Get a destination point from the direction
        let destination = self.get_destination(&direction);
        // If the cursor is on a hypercube, store the direction of swappage.
        if let Gem::Hypercube(_) = self.data[self.point_to_index(self.cursor)] {
            // Hypercubes matching with hypercubes destroy whole boards.
            if let Gem::Hypercube(_) = self.data[self.point_to_index(destination)] {
                self.data[self.point_to_index(self.cursor)] = Gem::Hypercube(GemSelector::All);
            } else {
                self.data[self.point_to_index(self.cursor)] = Gem::Hypercube(GemSelector::Color(
                    self.color_at_point(&self.data, destination).unwrap(),
                ));
            }
        }
        // If we are swapping *with* a hypercube, store the direction of swappage.
        else if let Gem::Hypercube(_) = self.data[self.point_to_index(destination)] {
            self.data[self.point_to_index(destination)] = Gem::Hypercube(GemSelector::Color(
                self.color_at_point(&self.data, self.cursor).unwrap(),
            ));
        }
        // Otherwise swap the gems
        else {
            self.swap_explicit(self.cursor, destination)
        }
    }

    /// Gets the width (=height) of the board.
    pub fn get_width(&self) -> usize {
        f32::sqrt(self.data.len() as f32) as usize
    }

    /// Swaps a gem with any other gem. `source` and `destination` are 2d coordinates.
    pub fn swap_explicit(&mut self, source: Point<usize>, destination: Point<usize>) {
        let destination_index = self.point_to_index(destination);
        let source_index = self.point_to_index(source);
        self.data.swap(destination_index, source_index);
    }

    /// Returns true if a point [x,y] is in the board.
    pub fn is_in_board(&self, point: Point<usize>) -> bool {
        point.1 < self.get_width() && point.0 < self.get_width()
    }

    /// Finds all normal gems that match and returns their positions in the board.
    /// This can be used as a "dry run" to highlight any gems that have been matched.
    pub fn get_matching_gems(&self) -> Vec<Point<usize>> {
        let mut valid_gems: Vec<Point<usize>> = Vec::new();
        for i in 0..self.data.len() {
            let point = self.index_to_point(i);
            if let Gem::Normal(_) = self.data[i] {
                if self.is_matching_gem(self.data.as_ref(), point) && !valid_gems.contains(&point) {
                    valid_gems.push(point);
                }
            }
        }
        valid_gems
    }

    /// Finds all the gems that are to be removed because of a special gem matching.
    pub fn get_matching_special_gems(&self) -> Vec<Point<usize>> {
        let mut valid_gems: Vec<Point<usize>> = Vec::new();
        let mut special_gems_found: Vec<Point<usize>> = Vec::new();
        for i in 0..self.data.len() {
            self.activate_special_gem(i, true).iter().for_each(|x| {
                let index = self.point_to_index(*x);
                match self.data[index] {
                    Gem::Normal(_) => valid_gems.push(*x),
                    _ => {
                        valid_gems.push(*x);
                        if index != i {
                            special_gems_found.push(*x);
                        }
                    }
                }
            });
        }
        // Iteratively loop to make sure we activated all gems recursively
        while !special_gems_found.is_empty() {
            let mut special_gems_new: Vec<Point<usize>> = Vec::new();
            special_gems_found.iter().for_each(|special_gem| {
                let special_gem_index = self.point_to_index(*special_gem);
                self.activate_special_gem(special_gem_index, false)
                    .iter()
                    .for_each(|x| {
                        let index = self.point_to_index(*x);
                        match self.data[index] {
                            Gem::Normal(_) => {
                                if !valid_gems.contains(x) {
                                    valid_gems.push(*x);
                                }
                            }
                            _ => {
                                if index != special_gem_index && !valid_gems.contains(x) {
                                    special_gems_new.push(*x);
                                } else if !valid_gems.contains(x) {
                                    valid_gems.push(*x);
                                }
                            }
                        }
                    });
            });
            special_gems_found = special_gems_new;
        }
        valid_gems
    }

    /// Given a special gem, returns all the gems it is to remove (including itself)
    /// If the gem specified by the index is not a special gem, the returning vector will be empty.
    /// index: The index of the special gem in self.data.
    /// need_matching: Whether the gem needs to be matching with other gems to be activated - set this to false in order to force the gem to be activated.
    pub fn activate_special_gem(&self, index: usize, need_matching: bool) -> Vec<Point<usize>> {
        let point = self.index_to_point(index);
        let mut to_remove: Vec<Point<usize>> = Vec::new();
        match self.data[index] {
            Gem::Hypercube(gem_selector) => match gem_selector {
                GemSelector::Color(color) => {
                    for i in 0..self.data.len() {
                        let piece_color = Board::color_at_index(&self.data, i);
                        if piece_color.is_some() && piece_color.unwrap() == color {
                            to_remove.push(self.index_to_point(i));
                        }
                    }
                    to_remove.push(point);
                }
                GemSelector::All => {
                    for i in 0..self.data.len() {
                        to_remove.push(self.index_to_point(i));
                    }
                    to_remove.push(point);
                }
                GemSelector::None => {}
            },
            Gem::Flame(_) => {
                if !need_matching || self.is_matching_gem(self.data.as_ref(), point) {
                    // Add the flame gem to the list of matching gems...
                    to_remove.push(point);
                    // And add all adjacent gems.
                    [
                        Point(point.0 as i32 - 1, point.1 as i32),
                        Point(point.0 as i32 - 1, point.1 as i32 - 1),
                        Point(point.0 as i32, point.1 as i32 - 1),
                        Point(point.0 as i32 + 1, point.1 as i32),
                        Point(point.0 as i32 + 1, point.1 as i32 + 1),
                        Point(point.0 as i32, point.1 as i32 + 1),
                        Point(point.0 as i32 + 1, point.1 as i32 - 1),
                        Point(point.0 as i32 - 1, point.1 as i32 + 1),
                    ]
                    .iter()
                    .for_each(|point| {
                        if point.0 >= 0 && point.1 >= 0 {
                            let point_usize = Point(point.0 as usize, point.1 as usize);
                            if self.is_in_board(point_usize) && !to_remove.contains(&point_usize) {
                                to_remove.push(point_usize);
                            }
                        }
                    })
                }
            }
            _ => {}
        };
        to_remove
    }

    /// Finds all matching gems, and then:
    /// - Removes them, replacing them with empty spaces.
    /// - Adds points for each matching gem.
    /// - Adds special gems if applicable.
    /// Returns a vector of the positions of all special gems that have been added.
    pub fn update_matching_gems(&mut self) -> Vec<Point<usize>> {
        let mut matching_gems = self.get_matching_gems();
        // Check for special gems
        // One-directional chains (flame gems, hypercubes, supernova gems)
        let mut chains: Vec<Vec<Point<usize>>> = Vec::new();
        matching_gems.iter().for_each(|point| {
            let mut chain_found = false;
            chains.iter_mut().for_each(|chain| {
                let first = chain.first().unwrap();
                let last = chain.last().unwrap();
                if self.data[self.point_to_index(*point)] == self.data[self.point_to_index(*first)]
                {
                    // Horizontal chain
                    if point.1 == first.1
                        && ((point.0 as i32 - first.0 as i32).abs() == 1
                            || (point.0 as i32 - last.0 as i32).abs() == 1)
                    {
                        chain.push(*point);
                        chain_found = true;
                    }
                    // Vertical chain
                    else if point.0 == first.0
                        && ((point.1 as i32 - first.1 as i32).abs() == 1
                            || (point.1 as i32 - last.1 as i32).abs() == 1)
                    {
                        chain.push(*point);
                        chain_found = true;
                    }
                }
            });
            // if no chains have been found, create a new one
            if !chain_found {
                chains.push(vec![*point]);
            }
        });
        let data_clone = self.data;
        // Set every matching gem and (matching) special gem to empty
        matching_gems.append(&mut self.get_matching_special_gems());
        matching_gems.iter().for_each(|point| {
            self.data[self.point_to_index(*point)] = Gem::Empty;
            self.score += POINTS_SWAP as u32;
            self.level_progress += self.get_swap_progress();
        });
        // Iterate over the chains and add special gems.
        let mut points = vec![];
        chains.iter().for_each(|chain| {
            if chain.len() == 4 {
                // TODO: Create gems where *they were matched*
                self.data[self.point_to_index(chain[1])] =
                    Gem::Flame(self.color_at_point(&data_clone, chain[0]).unwrap());
                points.push(chain[1]);
            }
            if chain.len() == 5 {
                self.data[self.point_to_index(chain[2])] = Gem::Hypercube(GemSelector::None);
                points.push(chain[2]);
            }
        });
        points
    }

    /// Returns true if the entire board is filled with gems.
    pub fn is_full(&self) -> bool {
        !self.data.iter().any(|x| Gem::Empty == *x)
    }

    /// Returns true if you can make a move on the board.
    pub fn is_valid(&self) -> bool {
        for i in 0..self.data.len() {
            let point = self.index_to_point(i);
            if self.data[i] != Gem::Empty && self.is_valid_gem(point) {
                return true;
            }
        }
        false
    }

    /// Returns true if you can make a move on a spot.
    pub fn is_valid_gem(&self, point: Point<usize>) -> bool {
        // If we swapped the piece, would we swap it outside the board? Check each direction to make sure you even *can* swap the piece.
        // also: Hypercubes can be matched with anything!
        if let Gem::Hypercube(_) = self.data[self.point_to_index(point)] {
            true
        } else {
            self.is_valid_move(point, Direction::Left)
                || self.is_valid_move(point, Direction::Right)
                || self.is_valid_move(point, Direction::Up)
                || self.is_valid_move(point, Direction::Down)
        }
    }

    /// Returns true if you can swap a gem, given the gem and direction of swappage.
    pub fn is_valid_move(&self, point: Point<usize>, direction: Direction) -> bool {
        // Ensure that we aren't subtracting from a (0,0)
        if point.0 == 0 && direction == Direction::Left
            || point.1 == 0 && direction == Direction::Up
        {
            false
        } else {
            // Store the destination coordinates
            let destination = self.get_destination_from(point, &direction);
            // 1. Check if the cursor and destination are in the map.
            if self.is_in_board(self.cursor) && self.is_in_board(destination) {
                // 2. Copy the board
                let mut data_copy = self.data;
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
    pub fn is_matching_gem(&self, data: &[Gem], point: Point<usize>) -> bool {
        let point_index = self.point_to_index(point);
        let point_color = Board::color_at_index(data, point_index);
        // Two pieces to the left
        if point.0 >= 2
            && self.color_at_point(data, point - Point(1, 0)) == point_color
            && self.color_at_point(data, point - Point(2, 0)) == point_color
        {
            true
        // Two pieces to the right
        } else if self.is_in_board(point + Point(2, 0))
            && self.color_at_point(data, point + Point(1, 0)) == point_color
            && self.color_at_point(data, point + Point(2, 0)) == point_color
        {
            true
        // Two pieces below it
        } else if self.is_in_board(point + Point(0, 2))
            && self.color_at_point(data, point + Point(0, 1)) == point_color
            && self.color_at_point(data, point + Point(0, 2)) == point_color
        {
            true
        // Two pieces above it
        } else if point.1 >= 2
            && self.color_at_point(data, point - Point(0, 1)) == point_color
            && self.color_at_point(data, point - Point(0, 2)) == point_color
        {
            true
        // Horizontal middle
        } else if point.0 >= 1
            && self.is_in_board(point + Point(1, 0))
            && self.color_at_point(data, point - Point(1, 0)) == point_color
            && self.color_at_point(data, point + Point(1, 0)) == point_color
        {
            true
        // Vertical middle
        } else if point.1 >= 1
            && self.is_in_board(point - Point(0, 1))
            && self.is_in_board(point + Point(0, 1))
            && self.color_at_point(data, point - Point(0, 1)) == point_color
            && self.color_at_point(data, point + Point(0, 1)) == point_color
        {
            true
        // Special gems!
        } else {
            matches!(self.data[point_index], Gem::Hypercube(_))
        }
    }

    /// Moves the cursor by 1 in an adjacent direction to it.
    pub fn move_cursor(&mut self, direction: Direction) {
        let destination = self.get_destination(&direction);
        if self.is_in_board(destination) {
            self.cursor = destination;
        }
    }

    /// Sets the cursor to a particular point.
    pub fn set_cursor(&mut self, point: Point<usize>) {
        self.cursor = point;
    }

    /// Gets the coordinates of the cursor
    pub fn get_cursor(&self) -> Point<usize> {
        self.cursor
    }

    /// Gets the destination of a swap, given the cursor and a destination
    pub fn get_destination(&self, direction: &Direction) -> Point<usize> {
        self.get_destination_from(self.cursor, direction)
    }

    /// Gets the destination of a swap, given a point and direction
    pub fn get_destination_from(&self, from: Point<usize>, direction: &Direction) -> Point<usize> {
        match direction {
            Direction::Left => from - Point(1, 0),
            Direction::Right => from + Point(1, 0),
            Direction::Up => from - Point(0, 1),
            Direction::Down => from + Point(0, 1),
        }
    }

    /// Returns a reference to `self.data`
    pub fn as_ref(&self) -> &[Gem] {
        self.data.as_ref()
    }

    /// Returns a reference to Board::config
    pub fn config_ref(&self) -> &BoardConfig {
        &self.config
    }

    /// Converts a Point to an index in self.data.
    pub fn point_to_index(&self, point: Point<usize>) -> usize {
        point.1 * self.get_width() + point.0
    }

    /// Converts an index in self.data to a Point.
    pub fn index_to_point(&self, index: usize) -> Point<usize> {
        let row = index / self.get_width();
        Point(index - row * self.get_width(), row)
    }

    /// Gets a gem color at an index.
    pub fn color_at_index(data: &[Gem], index: usize) -> Option<GemColor> {
        match data[index] {
            Gem::Normal(x) => Some(x),
            Gem::Flame(x) => Some(x),
            Gem::Star(x) => Some(x),
            Gem::Supernova(x) => Some(x),
            _ => None,
        }
    }

    /// Gets a gem color at a point.
    pub fn color_at_point(&self, data: &[Gem], point: Point<usize>) -> Option<GemColor> {
        Board::color_at_index(data, self.point_to_index(point))
    }
}
