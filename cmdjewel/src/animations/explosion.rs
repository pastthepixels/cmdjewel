use cursive::Printer;
use rand::Rng;
use cmdjewel_core::gems::Gem;
use cmdjewel_core::point::Point;
use crate::animations::{Animation, EXPLOSION_GRAVITY};

/// Explosion animation.
pub struct Explosion {
    keyframe: usize,
    velocities: Vec<(f32, f32)>,
    positions: Vec<Point<f32>>,
}

impl Explosion {
    /// Creates a new explosion animation.
    /// Requires the number of gems on a board, and the force of the explosion.
    pub fn new(num_gems: usize, force: f32) -> Self {
        let mut rng = rand::rng();
        Explosion {
            keyframe: 0,
            velocities: (0..num_gems)
                .map(|_| (rng.random_range(-force..force), rng.random_range(-force..force)))
                .collect(),
            positions: Self::calculate_positions(num_gems),
        }
    }
}

impl Animation for Explosion {
    /// Increase the keyframe by one, and update it
    fn tick(&mut self) {
        self.keyframe += 1;
        self.velocities.iter_mut().for_each(|x| {
            x.1 += EXPLOSION_GRAVITY;
        });
    }

    /// Gets an array of offsets (from the normal position you would print a gem) as integer tuples
    fn get_offsets(&self) -> Vec<Point<i32>> {
        let mut offsets: Vec<Point<i32>> = Vec::new();
        for i in 0..self.positions.len() {
            offsets.push(Point(
                (self.velocities[i].0 * self.keyframe as f32 + self.positions[i].0) as i32,
                (self.velocities[i].1 * self.keyframe as f32 + self.positions[i].1) as i32,
            ))
        }
        offsets
    }

    fn get_max_keyframe(&self) -> usize {
        80
    }

    fn get_keyframe(&self) -> usize {
        self.keyframe
    }

    fn draw_background(
        &self,
        printer: &Printer,
        _: &[Gem],
        width: usize,
        board_offset: &Point<usize>,
    ) {
        printer.print_box(
            (board_offset.0, board_offset.1),
            (width * 3 + 2, width + 2),
            false,
        );
    }
}
