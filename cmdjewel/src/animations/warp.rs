use crate::animations::{Animation, WARP_KEYFRAMES, WARP_REPEL_DISTANCE};
use cmdjewel_core::gems::Gem;
use cmdjewel_core::point::Point;
use cursive::style::{Color, PaletteColor};
use cursive::Printer;
use rand::Rng;

/// Warp animation.
pub struct Warp {
    keyframe: usize,
    positions: Vec<Point<f32>>,
    velocities: Vec<Point<f32>>,
    circles: Vec<(usize, Color)>,
}

impl Warp {
    /// Creates a new explosion animation.
    /// Requires the number of gems on a board, and the force of the explosion.
    pub fn new(num_gems: usize, force: f32) -> Self {
        let mut rng = rand::rng();
        Warp {
            keyframe: 0,
            velocities: (0..num_gems)
                .map(|_| {
                    Point(
                        rng.random_range(-force..force),
                        rng.random_range(-force..force),
                    )
                })
                .collect(),
            positions: Self::calculate_positions(num_gems),
            circles: Vec::new(),
        }
    }

    /// Draws a circle with the midpoint circle algorithm, adapted from [Wikipedia](https://en.wikipedia.org/wiki/Midpoint_circle_algorithm).
    fn draw_circle(printer: &Printer, origin: Point<f32>, radius: i32, color: Color) {
        let mut t1 = radius as f32 / 16.0;
        let mut x = radius as f32;
        let mut y = 0.0;
        while !(x < y) {
            // Draws points
            [
                cursive::Vec2::new((origin.0 + x) as usize, (origin.1 - y) as usize),
                cursive::Vec2::new((origin.0 + x) as usize, (origin.1 + y) as usize),
                cursive::Vec2::new((origin.0 - x) as usize, (origin.1 - y) as usize),
                cursive::Vec2::new((origin.0 - x) as usize, (origin.1 + y) as usize),
                cursive::Vec2::new((origin.0 + y) as usize, (origin.1 - x) as usize),
                cursive::Vec2::new((origin.0 + y) as usize, (origin.1 + x) as usize),
                cursive::Vec2::new((origin.0 - y) as usize, (origin.1 - x) as usize),
                cursive::Vec2::new((origin.0 - y) as usize, (origin.1 + x) as usize),
            ]
            .iter()
            .for_each(|vec| {
                if vec.x != 0 && vec.y != 0 {
                    printer.with_color(
                        cursive::theme::ColorStyle::new(color, PaletteColor::Background),
                        |printer| printer.print(vec, "•"),
                    );
                }
            });
            // Increments
            y += 1.0;
            t1 += y;
            let t2 = t1 - x;
            if t2 >= 0.0 {
                t1 = t2;
                x -= 1.0;
            }
        }
    }
}

impl Animation for Warp {
    /// Increase the keyframe by one, and update it
    fn tick(&mut self) {
        self.keyframe += 1;
        // Get center of board
        let mut center = Point(f32::sqrt(self.positions.len() as f32) / 2.0, 0.0);
        center.1 = center.0;
        center.0 *= 3.0;
        // For each gem...
        for i in 0..self.positions.len() {
            let distance = Point::<f32>::distance_to(self.positions[i], center) + 0.5;
            if distance != 0.0 {
                // normalised
                let direction = Point(
                    (center.0 - self.positions[i].0) / distance,
                    (center.1 - self.positions[i].1) / distance,
                );
                // If the distance to the center is larger by some amount, repel it from the center.
                if distance > WARP_REPEL_DISTANCE
                    || self.get_max_keyframe() - self.keyframe < WARP_KEYFRAMES
                {
                    self.velocities[i].0 -= direction.0 * (1.0 / distance);
                    self.velocities[i].1 -= direction.1 * (1.0 / distance);
                }
                // Otherwise, attract it to the center.
                else {
                    self.velocities[i].0 += direction.0 * (1.0 / distance);
                    self.velocities[i].1 += direction.1 * (1.0 / distance);
                }
                // Apply the velocity to the position of the gem.
                self.positions[i].0 += self.velocities[i].0;
                self.positions[i].1 += self.velocities[i].1;
            }
        }
        // Add/expand circles
        self.circles = self
            .circles
            .iter_mut()
            .map(|circle| (circle.0 + 1, circle.1))
            .collect();
        if self.keyframe % 15 == 0 && (self.get_max_keyframe() - self.keyframe) > WARP_KEYFRAMES {
            let mut rng = rand::rng();
            let color = match rng.random_range(0..8) {
                1 => Color::Rgb(126, 158, 189),
                2 => Color::Rgb(213, 219, 230),
                3 => Color::Rgb(190, 96, 105),
                4 => Color::Rgb(233, 201, 138),
                5 => Color::Rgb(162, 188, 139),
                6 => Color::Rgb(207, 135, 111),
                7 => Color::Rgb(174, 174, 255),
                _ => Color::Rgb(67, 76, 94),
            };
            self.circles.push((0, color))
        }
        if self.circles.len() > 5 {
            self.circles.remove(0);
        }
    }

    /// Gets an array of offsets (from the normal position you would print a gem) as integer tuples
    fn get_offsets(&self) -> Vec<Point<i32>> {
        self.positions
            .iter()
            .map(|x| Point(x.0 as i32, x.1 as i32))
            .collect()
    }

    fn get_max_keyframe(&self) -> usize {
        150
    }

    fn get_keyframe(&self) -> usize {
        self.keyframe
    }

    fn draw_background(&self, printer: &Printer, _: &[Gem], _: usize, _: &Point<usize>) {
        self.circles.iter().for_each(|circle| {
            Warp::draw_circle(
                printer,
                Point((printer.size.x / 2) as f32, (printer.size.y / 2) as f32),
                circle.0 as i32,
                circle.1,
            );
        })
    }
}
