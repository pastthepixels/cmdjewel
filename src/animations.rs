use crate::game::Gem;
use crate::game::Point;
use crate::view::BoardView;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::theme::Color;
use cursive::theme::PaletteColor;
use cursive::Printer;
use rand::Rng;
use std::sync::Arc;

const EXPLOSION_GRAVITY: f32 = 0.04;

const WARP_REPEL_DISTANCE: f32 = 5.;

// How many keyframes are needed for the warp animation to push out everything
const WARP_KEYFRAMES: usize = 50;

/// Fullscreen animations.
pub struct AnimationView<T: Animation + 'static> {
    animation: T,
    data: Vec<Gem>,
    // WTF rust
    on_finish: Option<Arc<Box<dyn 'static + Fn(&mut cursive::Cursive) + Send + Sync>>>,
}

impl<T: Animation + 'static> AnimationView<T> {
    pub fn new(animation: T, data: Vec<Gem>) -> Self {
        AnimationView {
            animation,
            data,
            on_finish: None,
        }
    }

    pub fn with_on_finish<S>(self, s: S) -> Self
    where
        S: 'static + Fn(&mut cursive::Cursive) + Send + Sync,
    {
        AnimationView {
            animation: self.animation,
            data: self.data,
            on_finish: Some(Arc::new(Box::new(s))),
        }
    }

    /// Copied from game::Board
    pub fn get_width(&self) -> usize {
        f32::sqrt(self.data.len() as f32) as usize
    }
}

impl<T: Animation + 'static> cursive::view::View for AnimationView<T> {
    fn draw(&self, printer: &Printer) {
        // TODO: we can't actually find where the top corner of the board is in global space. maybe one day we can change that.
        let board_offset = Point(
            printer.output_size.x / 2 - self.get_width() * 3 / 2 - 1,
            printer.output_size.y / 2 - self.get_width() / 2 - 1,
        );
        // Draws a background for the animation (if applicable)
        self.animation
            .draw_background(printer, &self.data, self.get_width(), &board_offset);
        // Gets all offsets
        let offsets = self.animation.get_offsets();
        // Loops through/prints NON-EMPTY gems
        for (i, gem) in offsets.iter().enumerate().take(self.data.len()) {
            let point = Point(
                (gem.0 + board_offset.0 as i32) as usize,
                (gem.1 + board_offset.1 as i32) as usize,
            );
            // Prints it
            if self.data[i] != Gem::Empty {
                printer.with_color(BoardView::gem_color(self.data[i]), |printer| {
                    printer.print((point.0, point.1), &BoardView::gem_string(self.data[i]))
                });
            }
        }
    }
    fn on_event(&mut self, event: Event) -> EventResult {
        match event {
            Event::Refresh => {
                self.animation.tick();
                if self.animation.get_keyframe() == self.animation.get_max_keyframe() {
                    // I hate multithreading
                    let on_finish = self.on_finish.clone();
                    EventResult::with_cb(move |s| {
                        s.pop_layer();
                        on_finish.clone().unwrap_or(Arc::new(Box::new(|_| {})))(s);
                    })
                } else {
                    EventResult::consumed()
                }
            }
            _ => EventResult::Ignored,
        }
    }
}

/// Trait all animations must implement
pub trait Animation: Send + Sync {
    fn tick(&mut self);
    fn get_offsets(&self) -> Vec<Point<i32>>;
    fn get_max_keyframe(&self) -> usize;
    fn get_keyframe(&self) -> usize;
    fn draw_background(&self, _: &Printer, _: &[Gem], _: usize, _: &Point<usize>);
    /// Gets the position on screen for each gem, relative to the top left of the board
    fn calculate_positions(num_gems: usize) -> Vec<Point<f32>> {
        let mut positions: Vec<Point<f32>> = Vec::new();
        let width = f32::sqrt(num_gems as f32) as usize;
        for i in 0..num_gems {
            let y = i / width;
            positions.push(Point((i - y * width) as f32 * 3.0, y as f32));
        }
        positions
    }
}

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
        let mut rng = rand::thread_rng();
        Explosion {
            keyframe: 0,
            velocities: (0..num_gems)
                .map(|_| (rng.gen_range(-force..force), rng.gen_range(-force..force)))
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
        let mut rng = rand::thread_rng();
        Warp {
            keyframe: 0,
            velocities: (0..num_gems)
                .map(|_| Point(rng.gen_range(-force..force), rng.gen_range(-force..force)))
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
            let mut rng = rand::thread_rng();
            let color = match rng.gen_range(0..8) {
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
        256
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
