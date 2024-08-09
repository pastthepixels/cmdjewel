// Handles game logic.

/// Cursor modes
enum CursorMode {
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
