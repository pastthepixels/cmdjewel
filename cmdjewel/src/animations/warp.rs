use crate::animations::{Animation, WARP_KEYFRAMES, WARP_PULL, WARP_SPIN};
use cmdjewel_core::gems::Gem;
use cmdjewel_core::point::Point;
use cursive::style::{Color, PaletteColor};
use cursive::Printer;
use rand::Rng;

/// Warp animation.
pub struct Warp {
    keyframe: usize,
    positions: Vec<Point<f32>>,
    circles: Vec<(usize, Color)>,
}

impl Warp {
    /// Creates a new explosion animation.
    /// Requires the number of gems on a board, and the force of the explosion.
    pub fn new(num_gems: usize, force: f32) -> Self {
        Warp {
            keyframe: 0,
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
        self.positions.iter_mut().for_each(|p| {
            // Translate
            p.0 -= center.0;
            p.1 -= center.1;
            // Rotate
            let mut h = (p.0.powi(2) + p.1.powi(2)).sqrt() - WARP_PULL;
            let u = (p.1 / p.0).atan() + h / WARP_SPIN;
            if p.0 < 0. {
                h = -h;
            }
            p.0 = h * u.cos();
            p.1 = h * u.sin();
            // Translate
            p.0 += center.0;
            p.1 += center.1;
            // Whups!! Edge case!!!
            if h.abs() < 0.5 {
                p.0 = 10000.;
                p.1 = 10000.;
            }
        });
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
