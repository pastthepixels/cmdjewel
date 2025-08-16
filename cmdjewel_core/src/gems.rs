use rand::Rng;
use rand::distributions::{Distribution, Standard};

/// Types of gems to use.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Gem {
    Empty,
    // Normal gems.
    Normal(GemColor),
    // Flame gems.
    Flame(GemColor),
    // Star gems.
    Star(GemColor),
    // Suprnova gems.
    Supernova(GemColor),
    // Hypercube
    Hypercube(GemSelector),
}

/// Gem colors. These are not associated with any special abilities nor do they include special gems (e.g. hypercubes)
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GemColor {
    Blue,
    White,
    Red,
    Yellow,
    Green,
    Orange,
    Purple,
}

/// Enum for different (general) ways of selecting gems on a board.
/// I mean I could also like add something with a vec of points if I want I guess
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GemSelector {
    Color(GemColor),
    All,
    None,
}

impl Distribution<Gem> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gem {
        Gem::Normal(match rng.gen_range(0..=6) {
            0 => GemColor::Blue,
            1 => GemColor::White,
            2 => GemColor::Red,
            3 => GemColor::Yellow,
            4 => GemColor::Green,
            5 => GemColor::Orange,
            _ => GemColor::Purple,
        })
    }
}
