use crate::game::Board;
use crate::game::{self};
use cursive::direction::Direction;
use cursive::event::{Event, EventResult, MouseButton, MouseEvent};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::view::CannotFocus;
use cursive::views::Dialog;
use cursive::{Cursive, Printer, Vec2};

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
    board: Board,
    has_focus: bool,
    pub cursor_mode: CursorMode,
}

impl BoardView {
    pub fn new() -> Self {
        BoardView {
            board: Board::new(),
            has_focus: false,
            cursor_mode: CursorMode::Normal,
        }
    }
}

impl BoardView {
    fn attempt_swap(&mut self, direction: game::Direction) {
        if self
            .board
            .is_valid_move(self.board.get_cursor(), direction.clone())
        {
            self.board.swap(direction.clone());
        }
        self.cursor_mode = CursorMode::Normal;
    }
}

impl cursive::view::View for BoardView {
    fn draw(&self, printer: &Printer) {
        for i in 0..self.board.as_ref().len() {
            let string = match self.board.as_ref()[i] {
                game::Gems::Empty => "•",
                game::Gems::Blue => "▼",
                game::Gems::White => "●",
                game::Gems::Red => "■",
                game::Gems::Yellow => "◆",
                game::Gems::Green => "⬟",
                game::Gems::Orange => "⬢",
                game::Gems::Purple => "▲",
            };
            let point = self.board.index_to_point(i);
            let mut color = match self.board.as_ref()[i] {
                game::Gems::Empty => (Color::Rgb(67, 76, 94), Color::Rgb(46, 52, 64)),
                game::Gems::Blue => (Color::Rgb(126, 158, 189), Color::Rgb(46, 52, 64)),
                game::Gems::White => (Color::Rgb(213, 219, 230), Color::Rgb(46, 52, 64)),
                game::Gems::Red => (Color::Rgb(190, 96, 105), Color::Rgb(46, 52, 64)),
                game::Gems::Yellow => (Color::Rgb(233, 201, 138), Color::Rgb(46, 52, 64)),
                game::Gems::Green => (Color::Rgb(162, 188, 139), Color::Rgb(46, 52, 64)),
                game::Gems::Orange => (Color::Rgb(207, 135, 111), Color::Rgb(46, 52, 64)),
                game::Gems::Purple => (Color::Rgb(174, 174, 255), Color::Rgb(46, 52, 64)),
            };
            if i == self.board.point_to_index(self.board.get_cursor()) {
                color = match self.cursor_mode {
                    CursorMode::Normal => (Color::Rgb(46, 52, 64), Color::Rgb(213, 219, 230)),
                    CursorMode::Swap => (Color::Rgb(213, 219, 230), Color::Rgb(190, 96, 105)),
                }
            }
            if !self.has_focus {
                color = (Color::Rgb(76, 86, 106), Color::Rgb(59, 66, 82))
            }
            printer.with_color(ColorStyle::new(color.0, color.1), |printer| {
                printer.print((point.0 * 3, point.1), &format!(" {} ", string))
            });
        }
    }

    fn take_focus(&mut self, _: Direction) -> Result<EventResult, CannotFocus> {
        Ok(EventResult::Consumed(None))
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        self.has_focus = true;
        self.board.update_matching_gems();
        self.board.fill_from_top();
        self.board.update_physics_frame();
        match event {
            Event::Char(c) => match c.to_ascii_lowercase() {
                ' ' => {
                    if let CursorMode::Normal = self.cursor_mode {
                        self.cursor_mode = CursorMode::Swap
                    } else {
                        self.cursor_mode = CursorMode::Normal
                    }
                    EventResult::consumed()
                }
                _ => {
                    return EventResult::with_cb(move |s| {
                        s.add_layer(Dialog::info("Key not recognized. Use the arrow keys to move and the enter key to enter SWAP mode."));
                    })
                }
            },
            Event::Key(cursive::event::Key::Left) => match self.cursor_mode {
                CursorMode::Swap => {
                    self.attempt_swap(game::Direction::Left);
                    EventResult::Consumed(None)
                }
                CursorMode::Normal => {
                    if self.board.get_cursor().0 != 0 {
                        self.board.move_cursor(game::Direction::Left);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
            },
            Event::Key(cursive::event::Key::Right) => match self.cursor_mode {
                CursorMode::Swap => {
                    self.attempt_swap(game::Direction::Right);
                    EventResult::Consumed(None)
                }
                CursorMode::Normal => {
                    if self.board.get_cursor().0 != self.board.get_width() - 1 {
                        self.board.move_cursor(game::Direction::Right);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
            },
            Event::Key(cursive::event::Key::Up) => match self.cursor_mode {
                CursorMode::Swap => {
                    self.attempt_swap(game::Direction::Up);
                    EventResult::Consumed(None)
                }
                CursorMode::Normal => {
                    if self.board.get_cursor().1 != 0 {
                        self.board.move_cursor(game::Direction::Up);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
            },
            Event::Key(cursive::event::Key::Down) => match self.cursor_mode {
                CursorMode::Swap => {
                    self.attempt_swap(game::Direction::Down);
                    EventResult::Consumed(None)
                }
                CursorMode::Normal => {
                    if self.board.get_cursor().1 != self.board.get_width() - 1 {
                        self.board.move_cursor(game::Direction::Down);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
            },
            Event::Key(cursive::event::Key::Enter) => {
                if let CursorMode::Normal = self.cursor_mode {
                    self.cursor_mode = CursorMode::Swap
                } else {
                    self.cursor_mode = CursorMode::Normal
                }
                EventResult::consumed()
            }
            Event::FocusLost => {
                self.has_focus = false;
                EventResult::Ignored
            }
            _ => EventResult::Ignored,
        }
    }

    fn required_size(&mut self, _: Vec2) -> Vec2 {
        (self.board.get_width() * 3, self.board.get_width()).into()
    }
}
