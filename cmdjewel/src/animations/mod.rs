/// Fullscreen animation types
pub mod explosion;
pub mod warp;

use cmdjewel_core::gems::Gem;
use cmdjewel_core::point::Point;
use crate::view::BoardView;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::Printer;
use std::sync::Arc;

const EXPLOSION_GRAVITY: f32 = 0.04;

const WARP_REPEL_DISTANCE: f32 = 5.;

// How many keyframes are needed for the warp animation to push out everything
const WARP_KEYFRAMES: usize = 50;

/// Enum containing types of animations (and any data)
#[derive(PartialEq, Eq)]
pub enum AnimationType {
    Blink(bool),
    Highlight,
    Explosion,
    Warp,
}

/// Describes an animation
pub struct AnimationDetails {
    /// Point corresponding with a cell to animate.
    pub point: Point<usize>,
    /// Length of a cell animation
    pub duration: u8,
    pub animation_type: AnimationType,
}

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
