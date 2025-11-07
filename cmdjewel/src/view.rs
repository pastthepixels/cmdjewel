use crate::animations::{AnimationDetails, AnimationType, AnimationView};
use crate::constants::strings;
use crate::{config, constants, ui};
use cmdjewel_core::board::{Board, BoardConfig};
use cmdjewel_core::gems::Gem;
use cmdjewel_core::point;
use cmdjewel_core::point::Point;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseEvent};
use cursive::theme::{Color, ColorStyle, PaletteColor};
use cursive::traits::Resizable;
use cursive::view::CannotFocus;
use cursive::views::{Dialog, ProgressBar, TextView};
use cursive::{Printer, Vec2};

/// Cursor modes
pub enum CursorMode {
    Normal,
    Swap,
}

impl std::fmt::Display for CursorMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorMode::Normal => write!(f, "NORMAL"),
            CursorMode::Swap => write!(f, "SWAP"),
        }
    }
}

pub struct BoardView {
    pub(crate) board: Board,
    has_focus: bool,
    animations: Vec<AnimationDetails>,
    cursor_down: Point<usize>,
    pub cursor_mode: CursorMode,
    pub autoplay: bool,
    pub animations_enabled: bool,
}

impl BoardView {
    pub fn new(config: BoardConfig) -> Self {
        BoardView {
            board: config::new_board(config),
            has_focus: false,
            animations: Vec::new(),
            cursor_mode: CursorMode::Normal,
            autoplay: false,
            animations_enabled: true,
            cursor_down: Point(0, 0),
        }
    }

    /// Sets the cursor to the first swappable gem
    pub fn hint(&mut self) {
        for i in 0..self.board.as_ref().len() {
            let point = self.board.index_to_point(i);
            if self.board.is_valid_gem(point) {
                self.board.set_cursor(point);
                break;
            }
        }
    }

    /// Adds an AnimationDetails for an explosion.
    /// This effectively makes a query for the BoardView to make a fullscreen explosion animation.
    pub fn animation_explode(&mut self) {
        self.animations.push(AnimationDetails {
            point: Point(0, 0),
            duration: 10,
            animation_type: AnimationType::Explosion,
        });
    }

    /// Adds an AnimationDetails for a warp animation.
    /// This effectively makes a query for the BoardView to make a fullscreen warp animation.
    pub fn animation_warp(&mut self) {
        self.animations.push(AnimationDetails {
            point: Point(0, 0),
            duration: 2,
            animation_type: AnimationType::Warp,
        });
    }

    /// Swap two gems at the cursor in a given direction--but only if a valid move is possible.
    fn attempt_swap(&mut self, direction: point::Direction) {
        if self.board.is_valid_move(self.board.get_cursor(), direction) {
            self.board.swap(direction);
        }
        self.cursor_mode = CursorMode::Normal;
    }

    /// Updates all animations. Animations are automatically destroyed in the event loop.
    fn update_animations(&mut self) {
        // Reduce duration of each animation
        self.animations.iter_mut().for_each(|animation| {
            // Animation logic
            match animation.animation_type {
                AnimationType::Blink(s) => {
                    if animation.duration % 2 == 0 {
                        animation.animation_type = AnimationType::Blink(!s)
                    }
                }
                _ => (),
            }
            // Count down all animations
            if animation.duration != 0 {
                animation.duration -= 1;
            }
        });
    }

    /// Handles the creation of all animations.
    /// This is called:
    /// - When `BoardView.animations` is empty.
    /// - If `BoardView.animations_enabled = true`.
    /// - Before `update_board()`.
    fn create_animations(&mut self) {
        // Highlight all matching gems
        if self.board.is_full() {
            let mut points: Vec<Point<usize>> = Vec::new();
            // Highlight matching gems
            self.board.get_matches().iter().for_each(|m| {
                m.gems.iter().for_each(|&p| {
                    if !points.contains(&p) {
                        if let Gem::Normal(_) = self.board.get_gem(p.clone()) {
                            // Highlight normal gems
                            self.animations.push(AnimationDetails {
                                point: p,
                                duration: 8,
                                animation_type: AnimationType::Highlight,
                            });
                        } else {
                            // Blink special gems
                            self.animations.push(AnimationDetails {
                                point: p,
                                duration: 16,
                                animation_type: AnimationType::Blink(true),
                            });
                        }
                        points.push(p);
                    }
                    // TODO: recursively iterate over children (soon™)
                })
            });
        }
        // Explode if not valid
        if !self.board.is_valid() && self.board.is_full() {
            self.animation_explode();
        }
        // TODO: get inserted power gems, and make them blink.
    }

    /// Updates board logic.
    fn update_board(&mut self) {
        // TODO: get inserted gems
        if self.board.is_buffer_empty() {
            if self.board.is_full() {
                self.board.update_matching_gems();
                /*
                let inserted = self.board.update_matching_gems();
                inserted.iter().for_each(|p| {
                    // Blinks inserted gems
                    if self.animations_enabled {
                        self.animations.push(AnimationDetails {
                            point: *p,
                            duration: 16,
                            animation_type: AnimationType::Blink(true),
                        })
                    }
                });
                if inserted.len() > 0 {
                    return;
                }
                */
            } else {
                self.board.fill_gem_buffer();
            }
        }
        self.board.slide_down();
        self.board.update_level();
        if self.autoplay && self.board.is_full() {
            self.hint();
            self.attempt_swap(point::Direction::Left);
            self.attempt_swap(point::Direction::Right);
            self.attempt_swap(point::Direction::Up);
            self.attempt_swap(point::Direction::Down);
        }
    }

    /// Moves the cursor by 1 in any direction and returns an EventResult.
    fn move_cursor(&mut self, direction: point::Direction) -> EventResult {
        match self.cursor_mode {
            CursorMode::Swap => {
                self.attempt_swap(direction);
                EventResult::Consumed(None)
            }
            CursorMode::Normal => {
                let cursor_valid = match direction {
                    point::Direction::Left => self.board.get_cursor().0 != 0,
                    point::Direction::Right => {
                        self.board.get_cursor().0 != self.board.get_width() - 1
                    }
                    point::Direction::Up => self.board.get_cursor().1 != 0,
                    point::Direction::Down => {
                        self.board.get_cursor().1 != self.board.get_width() - 1
                    }
                };
                if cursor_valid {
                    self.board.move_cursor(direction);
                    EventResult::Consumed(None)
                } else {
                    EventResult::Ignored
                }
            }
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        let is_valid = !self
            .animations
            .iter()
            .any(|x| x.animation_type == AnimationType::Explosion);
        if !is_valid {
            return;
        }
        // Loop through each gem/cell
        for i in 0..self.board.as_ref().len() {
            let string = constants::gems::gem_string(self.board.as_ref()[i]);
            let point = self.board.index_to_point(i);
            let mut color = constants::gems::gem_color(self.board.as_ref()[i]);
            // Swap colors for highlighted gems.
            self.animations.iter().for_each(|anim| {
                if anim.point.0 == point.0
                    && anim.point.1 == point.1
                    && (anim.animation_type == AnimationType::Highlight
                        || anim.animation_type == AnimationType::Blink(true))
                {
                    color = color.invert();
                }
            });
            // If there's no animation happening, you can theme the cell under whatever conditions.
            if self.animations.is_empty() {
                // for instance, if the cursor is in the same position, set some custom colors.
                if i == self.board.point_to_index(self.board.get_cursor()) {
                    color = match self.cursor_mode {
                        CursorMode::Normal => {
                            ColorStyle::new(Color::Rgb(46, 52, 64), Color::Rgb(213, 219, 230))
                        }
                        CursorMode::Swap => {
                            ColorStyle::new(Color::Rgb(213, 219, 230), Color::Rgb(190, 96, 105))
                        }
                    }
                }
            }
            // If the board is not focused, grey out everything.
            if !self.has_focus {
                color = ColorStyle::new(PaletteColor::Secondary, PaletteColor::Tertiary);
            }
            // Print things, with spacing!
            printer.with_color(color, |printer| {
                printer.print((point.0 * 3, point.1), &format!(" {} ", string))
            });
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (self.board.get_width() * 3, self.board.get_width()).into()
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        // Handles losing focus
        if let Event::FocusLost = event {
            self.has_focus = false;
            return EventResult::Ignored;
        } else {
            self.has_focus = true;
        }
        // Handle events
        match event {
            Event::Mouse {
                offset,
                position,
                event,
            } => {
                // NOTE: 3 is some constant that represents the width of each gem
                let point: Point<i32> = Point(
                    (position.x as i32 - offset.x as i32) / 3,
                    position.y as i32 - offset.y as i32,
                );
                match event {
                    MouseEvent::Press(_) => {
                        if point.0 < 8 && point.1 < 8 && point.0 >= 0 && point.1 >= 0 {
                            self.cursor_down = Point(point.0 as usize, point.1 as usize);
                            self.board.set_cursor(self.cursor_down);
                            self.cursor_mode = CursorMode::Swap;
                            EventResult::Consumed(None)
                        } else {
                            EventResult::Ignored
                        }
                    }
                    MouseEvent::Release(_) => {
                        let (u, d, l, r) = {
                            (
                                (self.cursor_down.1 as i32 - point.1),
                                -(self.cursor_down.1 as i32 - point.1),
                                (self.cursor_down.0 as i32 - point.0),
                                -(self.cursor_down.0 as i32 - point.0),
                            )
                        };
                        let max = u.max(d).max(l).max(r);
                        let mut dir = if max == r {
                            Some(point::Direction::Right)
                        } else if max == l {
                            Some(point::Direction::Left)
                        } else if max == d {
                            Some(point::Direction::Down)
                        } else if max == u {
                            Some(point::Direction::Up)
                        } else {
                            None
                        };
                        if self.cursor_down.0 == point.0 as usize
                            && self.cursor_down.1 == point.1 as usize
                        {
                            dir = None;
                        }
                        if let Some(d) = dir {
                            self.attempt_swap(d);
                            EventResult::Consumed(None)
                        } else {
                            self.cursor_mode = CursorMode::Normal;
                            EventResult::Ignored
                        }
                    }
                    _ => EventResult::Ignored,
                }
            }
            Event::Refresh => {
                let mut initial_level = self.board.get_level() + 1;
                let is_valid = !self
                    .animations
                    .iter()
                    .any(|x| x.animation_type == AnimationType::Explosion);
                // Updates animations
                self.update_animations();
                let mut exists_running_animation = false;
                let mut is_animation_removed = false;
                for i in (0..self.animations.len()).rev() {
                    if self.animations[i].duration == 0 {
                        // Remove finished animations
                        self.animations.remove(i);
                        is_animation_removed = true;
                    } else {
                        exists_running_animation = true;
                    }
                }
                if !exists_running_animation {
                    if !is_animation_removed && self.animations_enabled {
                        self.create_animations();
                    }
                    // Update board
                    if self.animations.is_empty() {
                        self.update_board();
                    }
                }
                // Updates GUI (yes i have to make all these variables i love rust multithreading)
                let score = self.board.get_score();
                let level = self.board.get_level() + 1;
                let progress = self.board.get_level_progress() * 100.;
                // Hacks initial_level if there is a warp animation
                if self
                    .animations
                    .iter()
                    .any(|x| x.animation_type == AnimationType::Warp)
                {
                    initial_level = level + 1; // Now initial_level != level
                }
                // Hacks initial_level if we don't want to use animations
                if !self.animations_enabled {
                    initial_level = level;
                }
                EventResult::with_cb(move |s| {
                    s.call_on_name("score", |score_view: &mut TextView| {
                        score_view.set_content(format!("{}", score))
                    });
                    s.call_on_name("level", |level_view: &mut TextView| {
                        level_view.set_content(format!("{} {}", strings::LEVEL, level))
                    });
                    s.call_on_name("progress", |p: &mut ProgressBar| {
                        p.set_value(progress as usize)
                    });
                    // Explodes if applicable
                    if !is_valid {
                        let data = s
                            .call_on_name("board", |b: &mut BoardView| {
                                b.board.as_ref().to_vec() // Return board as vec
                            })
                            .unwrap();
                        s.screen_mut().add_fullscreen_layer(
                            AnimationView::new(
                                crate::animations::explosion::Explosion::new(data.len(), 1.0),
                                data,
                            )
                            .with_on_finish(move |s| {
                                s.add_layer(
                                    Dialog::text(strings::game_over(score, level))
                                        .button(strings::OK, |s| ui::show_menu_main(s)),
                                );
                            })
                            .full_screen(),
                        );
                    }
                    // Warps if available
                    if initial_level != level {
                        let data = s
                            .call_on_name("board", |b: &mut BoardView| b.board.as_ref().to_vec())
                            .unwrap();
                        s.screen_mut().add_fullscreen_layer(
                            AnimationView::new(
                                crate::animations::warp::Warp::new(data.len(), 1.0),
                                data,
                            )
                            .full_screen(),
                        )
                    }
                })
            }
            Event::Char(c) => match c.to_ascii_lowercase() {
                ' ' => {
                    if let CursorMode::Normal = self.cursor_mode {
                        self.cursor_mode = CursorMode::Swap
                    } else {
                        self.cursor_mode = CursorMode::Normal
                    }
                    EventResult::consumed()
                }
                ':' => EventResult::Ignored,
                'h' => self.move_cursor(point::Direction::Left),
                'l' => self.move_cursor(point::Direction::Right),
                'k' => self.move_cursor(point::Direction::Up),
                'j' => self.move_cursor(point::Direction::Down),
                _ => EventResult::with_cb(move |s| {
                    s.add_layer(Dialog::info(strings::KEY_NOT_FOUND));
                }),
            },
            Event::Key(cursive::event::Key::Left) => self.move_cursor(point::Direction::Left),
            Event::Key(cursive::event::Key::Right) => self.move_cursor(point::Direction::Right),
            Event::Key(cursive::event::Key::Up) => self.move_cursor(point::Direction::Up),
            Event::Key(cursive::event::Key::Down) => self.move_cursor(point::Direction::Down),
            Event::Key(cursive::event::Key::Enter) => {
                if let CursorMode::Normal = self.cursor_mode {
                    self.cursor_mode = CursorMode::Swap
                } else {
                    self.cursor_mode = CursorMode::Normal
                }
                EventResult::consumed()
            }
            _ => EventResult::Ignored,
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }
}
