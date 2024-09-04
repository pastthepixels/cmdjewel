use crate::animations::AnimationView;
use crate::game::{self};
use crate::game::{Board, BoardConfig};
use crate::ui;
use cursive::direction::Direction;
use cursive::event::{Event, EventResult};
use cursive::theme::{Color, ColorStyle};
use cursive::traits::Resizable;
use cursive::view::CannotFocus;
use cursive::views::{Dialog, ProgressBar, TextView};
use cursive::{Printer, Vec2};

/// Cursor modes
pub enum CursorMode {
    Normal,
    Swap,
}

/// Animations
#[derive(PartialEq, Eq)]
pub enum AnimationType {
    Highlight,
    Explosion,
    Warp,
}

pub struct Animation {
    pub point: game::Point<usize>,
    pub duration: u8,
    pub animation_type: AnimationType,
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
    board: Board,
    has_focus: bool,
    animations: Vec<Animation>,
    pub cursor_mode: CursorMode,
    pub autoplay: bool,
}

impl BoardView {
    pub fn new(config: BoardConfig) -> Self {
        BoardView {
            board: Board::new(config),
            has_focus: false,
            animations: Vec::new(),
            cursor_mode: CursorMode::Normal,
            autoplay: false,
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

    // Explodes the board
    pub fn animation_explode(&mut self) {
        self.animations.push(Animation {
            point: game::Point(0, 0),
            duration: 10,
            animation_type: AnimationType::Explosion,
        });
    }

    // Initiates the warp animation
    pub fn animation_warp(&mut self) {
        self.animations.push(Animation {
            point: game::Point(0, 0),
            duration: 2,
            animation_type: AnimationType::Warp,
        });
    }

    fn attempt_swap(&mut self, direction: game::Direction) {
        if self
            .board
            .is_valid_move(self.board.get_cursor(), direction.clone())
        {
            self.board.swap(direction.clone());
        }
        self.cursor_mode = CursorMode::Normal;
    }

    /// Updatess all animations. Animations are automatically destroyed in the event loop.
    fn update_animations(&mut self) {
        // Reduce duration of each animation
        self.animations.iter_mut().for_each(|animation| {
            if animation.duration != 0 {
                animation.duration -= 1;
            }
        });
    }

    /// Creates all animations.
    /// TODO: for moving the board, use reposition_layer
    fn create_animations(&mut self) {
        // Highlight all matching gems
        if self.board.is_full() {
            let mut points: Vec<game::Point<usize>> = Vec::new();
            self.board
                .get_matching_gems()
                .into_iter()
                .chain(self.board.get_matching_special_gems().into_iter())
                .for_each(|x| {
                    if !points.contains(&x) {
                        self.animations.push(Animation {
                            point: x,
                            duration: 8,
                            animation_type: AnimationType::Highlight,
                        });
                        points.push(x);
                    }
                });
        }
        // Explode if not valid
        if !self.board.is_valid() && self.board.is_full() {
            self.animation_explode();
        }
    }

    /// Updates board logic.
    fn update_board(&mut self) {
        if self.board.is_full() {
            self.board.update_matching_gems();
        }
        if self.board.is_buffer_empty() {
            self.board.fill_gem_buffer();
        }
        self.board.slide_down();
        self.board.update_level();
        //self.board.update_physics_frame();
        if self.autoplay && self.board.is_full() {
            self.hint();
            self.attempt_swap(game::Direction::Left);
            self.attempt_swap(game::Direction::Right);
            self.attempt_swap(game::Direction::Up);
            self.attempt_swap(game::Direction::Down);
        }
    }

    /// Moves the cursor by 1 in any direction and returns an EventResult.
    fn move_cursor(&mut self, direction: game::Direction) -> EventResult {
        match self.cursor_mode {
            CursorMode::Swap => {
                self.attempt_swap(direction);
                EventResult::Consumed(None)
            }
            CursorMode::Normal => {
                let cursor_valid = match direction {
                    game::Direction::Left => self.board.get_cursor().0 != 0,
                    game::Direction::Right => {
                        self.board.get_cursor().0 != self.board.get_width() - 1
                    }
                    game::Direction::Up => self.board.get_cursor().1 != 0,
                    game::Direction::Down => {
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

    // Generics

    /// Gets a printable string from a game::Gems.
    /// This doesn't belong in game.rs as that file only contains game logic and nothing user-facing.
    pub fn gem_string(gem: game::Gem) -> String {
        match gem {
            game::Gem::Empty => "•",
            game::Gem::Normal(x) => match x {
                game::GemColor::Blue => "▼",
                game::GemColor::White => "●",
                game::GemColor::Red => "■",
                game::GemColor::Yellow => "◆",
                game::GemColor::Green => "⬟",
                game::GemColor::Orange => "⬢",
                game::GemColor::Purple => "▲",
            },
            game::Gem::Flame(x) => match x {
                game::GemColor::Blue => "▽",
                game::GemColor::White => "○",
                game::GemColor::Red => "□",
                game::GemColor::Yellow => "◇",
                game::GemColor::Green => "⬠",
                game::GemColor::Orange => "⬡",
                game::GemColor::Purple => "△",
            },
            game::Gem::Star(_) => "★",
            game::Gem::Supernova(_) => "☆",
            game::Gem::Hypercube(_) => "◩",
        }
        .into()
    }

    /// Gets a ColorStyle given a game::Gems
    pub fn gem_color(gem: game::Gem) -> ColorStyle {
        match gem {
            game::Gem::Empty => ColorStyle::new(Color::Rgb(67, 76, 94), Color::Rgb(46, 52, 64)),
            game::Gem::Normal(x) => BoardView::colorstyle_from_gemcolor(x),
            game::Gem::Flame(x) => BoardView::colorstyle_from_gemcolor(x),
            game::Gem::Star(x) => BoardView::colorstyle_from_gemcolor(x),
            game::Gem::Supernova(x) => BoardView::colorstyle_from_gemcolor(x),
            game::Gem::Hypercube(_) => {
                ColorStyle::new(Color::Rgb(213, 219, 230), Color::Rgb(67, 76, 94))
            }
        }
    }

    /// Returns a ColorStyle from a game::GemColor
    fn colorstyle_from_gemcolor(gem_color: game::GemColor) -> ColorStyle {
        match gem_color {
            game::GemColor::Blue => {
                ColorStyle::new(Color::Rgb(126, 158, 189), Color::Rgb(46, 52, 64))
            }
            game::GemColor::White => {
                ColorStyle::new(Color::Rgb(213, 219, 230), Color::Rgb(46, 52, 64))
            }
            game::GemColor::Red => {
                ColorStyle::new(Color::Rgb(190, 96, 105), Color::Rgb(46, 52, 64))
            }
            game::GemColor::Yellow => {
                ColorStyle::new(Color::Rgb(233, 201, 138), Color::Rgb(46, 52, 64))
            }
            game::GemColor::Green => {
                ColorStyle::new(Color::Rgb(162, 188, 139), Color::Rgb(46, 52, 64))
            }
            game::GemColor::Orange => {
                ColorStyle::new(Color::Rgb(207, 135, 111), Color::Rgb(46, 52, 64))
            }
            game::GemColor::Purple => {
                ColorStyle::new(Color::Rgb(174, 174, 255), Color::Rgb(46, 52, 64))
            }
        }
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.board.as_ref().len() {
            let string = BoardView::gem_string(self.board.as_ref()[i]);
            let point = self.board.index_to_point(i);
            let mut color = BoardView::gem_color(self.board.as_ref()[i]);
            // Swap colors for highlighted gems.
            self.animations.iter().for_each(|anim| {
                if anim.point.0 == point.0
                    && anim.point.1 == point.1
                    && (*anim).animation_type == AnimationType::Highlight
                {
                    color = color.invert();
                }
            });
            // If there's no animation happening, you can theme the cell under whatever conditions.
            if self.animations.len() == 0 {
                // for instance, this is the cursor.
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
                color = ColorStyle::new(Color::Rgb(76, 86, 106), Color::Rgb(59, 66, 82))
            }
            printer.with_color(color, |printer| {
                printer.print((point.0 * 3, point.1), &format!(" {} ", string))
            });
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
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
            Event::Refresh => {
                let mut initial_level = self.board.get_level() + 1;
                // Updates animations
                self.update_animations();
                let mut exists_running_animation = false;
                let mut is_animation_removed = false;
                for i in (0..self.animations.len()).rev() {
                    if self.animations[i].duration == 0 {
                        self.animations.remove(i);
                        is_animation_removed = true;
                    } else {
                        exists_running_animation = true;
                    }
                }
                if !exists_running_animation {
                    if !is_animation_removed {
                        self.create_animations();
                    }
                    // Update board
                    if self.animations.len() == 0 {
                        self.update_board();
                    }
                }
                // Updates GUI (yes i have to make all these variables i love rust multithreading)
                let score = self.board.get_score();
                let level = self.board.get_level() + 1;
                let progress = self.board.get_level_progress() * 100.;
                let mut is_valid = self
                    .animations
                    .iter()
                    .find(|x| (**x).animation_type == AnimationType::Explosion)
                    .is_none();
                // Sets is_valid to true and shuffles if the board is not valid
                if is_valid == false && self.board.config_ref().infinite {
                    //self.board.shuffle();
                    is_valid = true;
                }
                // Hacks initial_level if there is a warp animation
                if self
                    .animations
                    .iter()
                    .find(|x| (**x).animation_type == AnimationType::Warp)
                    .is_some()
                {
                    initial_level = level + 1; // Now initial_level != level
                }
                EventResult::with_cb(move |s| {
                    s.call_on_name("score", |score_view: &mut TextView| {
                        score_view.set_content(format!("{}", score))
                    });
                    s.call_on_name("level", |level_view: &mut TextView| {
                        level_view.set_content(format!("Level {}", level))
                    });
                    s.call_on_name("progress", |p: &mut ProgressBar| {
                        p.set_value(progress as usize)
                    });
                    // Explodes if applicable
                    if is_valid == false {
                        let data = s
                            .call_on_name("board", |b: &mut BoardView| b.board.as_ref().to_vec())
                            .unwrap();
                        s.screen_mut().add_fullscreen_layer(
                            AnimationView::new(
                                crate::animations::Explosion::new(data.len(), 1.0),
                                data,
                            )
                            .with_on_finish(move |s| {
                                ui::show_menu_main(s);
                                s.add_layer(Dialog::info(&format!(
                                    "Game over! You scored {} points and got to level {}.",
                                    score, level
                                )));
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
                            AnimationView::new(crate::animations::Warp::new(data.len(), 1.0), data)
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
                'h' => self.move_cursor(game::Direction::Left),
                'l' => self.move_cursor(game::Direction::Right),
                'k' => self.move_cursor(game::Direction::Up),
                'j' => self.move_cursor(game::Direction::Down),
                _ => EventResult::with_cb(move |s| {
                    s.add_layer(Dialog::info("Key not recognized. Use the arrow keys to move and the enter key to enter SWAP mode."));
                }),
            },
            Event::Key(cursive::event::Key::Left) => self.move_cursor(game::Direction::Left),
            Event::Key(cursive::event::Key::Right) => self.move_cursor(game::Direction::Right),
            Event::Key(cursive::event::Key::Up) => self.move_cursor(game::Direction::Up),
            Event::Key(cursive::event::Key::Down) => self.move_cursor(game::Direction::Down),
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

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (self.board.get_width() * 3, self.board.get_width()).into()
    }
}
