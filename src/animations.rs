use crate::game::Board;
use crate::game::Gems;
use crate::game::Point;
use crate::view::BoardView;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::Printer;
use rand::Rng;
use std::sync::Arc;

const EXPLOSION_GRAVITY: f32 = 0.04;

/// Fullscreen animations.
pub struct AnimationView<T: Animation + 'static> {
    animation: T,
    data: Vec<Gems>,
    // WTF rust
    on_finish: Option<Arc<Box<dyn 'static + Fn(&mut cursive::Cursive) + Send + Sync>>>,
}

impl<T: Animation + 'static> AnimationView<T> {
    pub fn new(animation: T, data: Vec<Gems>) -> Self {
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
        // Prints a box as the board TODO: only in explosion animation
        printer.print_box(
            (board_offset.0, board_offset.1),
            (self.get_width() * 3 + 2, self.get_width() + 2),
            false,
        );
        // Loops through/prints NON-EMPTY gems
        for i in 0..self.data.len() {
            // Gets the original offset of the gem
            let mut point = Point(0, i / self.get_width());
            point.0 = (i - point.1 * self.get_width()) * 3;
            // Adds to it the offset in the animation
            let offset = self.animation.get_offsets()[i];
            point = Point(
                (point.0 as i32 + offset.0 + board_offset.0 as i32) as usize,
                (point.1 as i32 + offset.1 + board_offset.1 as i32) as usize,
            );
            // Prints it
            if self.data[i] != Gems::Empty {
                printer.with_color(BoardView::gem_color(self.data[i]), |printer| {
                    printer.print(
                        (point.0, point.1),
                        &format!("{}", BoardView::gem_string(self.data[i])),
                    )
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
    fn get_offsets(&self) -> Vec<(i32, i32)>;
    fn get_max_keyframe(&self) -> usize;
    fn get_keyframe(&self) -> usize;
}

/// Explosion animation.
/// Takes in a reference to a board and a keyframe, and returns a matrix the same size as the board's data which contains offsets for each gem.
pub struct Explosion {
    keyframe: usize,
    velocities: Vec<(f32, f32)>,
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
    fn get_offsets(&self) -> Vec<(i32, i32)> {
        self.velocities
            .iter()
            .map(|x| {
                (
                    (x.0 * self.keyframe as f32) as i32,
                    (x.1 * self.keyframe as f32) as i32,
                )
            })
            .collect()
    }

    fn get_max_keyframe(&self) -> usize {
        80
    }

    fn get_keyframe(&self) -> usize {
        self.keyframe
    }
}

/// Moves gems in a board corresponding to the warp animation. Does not create tunnels.
/// Takes in a reference to a board and a keyframe, and returns a matrix the same size as the board's data which contains offsets for each gem.
pub fn warp_gems(board: &Board, keyframe: usize) -> Vec<Point> {
    todo!()
}

/// Draws tunnels given a keyframe and reference to a cursive Printer
pub fn warp_tunnels(printer: &Printer, keyframe: usize) {
    todo!()
}
