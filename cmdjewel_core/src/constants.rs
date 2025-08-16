/// Specifies how much points you get for each gem successfully swapped.
pub const POINTS_SWAP: u8 = 30;

/// Specifies how much a swap counts toward progressing through each level, for each gem successfully swapped.
pub const PROGRESS_SWAP_INITIAL: f32 = 0.025;

/// PROGRESS_SWAP_INITIAL gets multiplied by this for each level
pub const PROGRESS_SWAP_FALLOFF: f32 = 0.9;

pub const PROGRESS_SWAP_MIN: f32 = 0.001;
