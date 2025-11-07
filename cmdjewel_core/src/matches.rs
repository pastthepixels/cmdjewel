use crate::{board::Board, gems::Gem, point::Point};

pub struct Match {
    /// A list of matched gems -- gems to remove from the board, or iterate to find a special gem in need of activation.
    pub gems: Vec<Point<usize>>,

    /// The point where the match takes place. The cursor's position, if it lands on a point in `self.gems`, or the first point in `self.gems` otherwise.
    pub at: Point<usize>,

    /// A gem that the match creates, if any.
    pub what: Option<Gem>,

    /// Subsequent matches caused by the match. (e.g., if a special gem was activated)
    pub children: Vec<Match>,
}

impl Match {
    pub fn new(gems: Vec<Point<usize>>) -> Match {
        Match {
            gems: gems.clone(),
            at: gems[0],
            what: None,
            children: vec![],
        }
    }
}

/// Do a vertical (or horizontal) scan for matches -- either scan each row for all vertical matches, or each column for all horizontal matches
pub fn scan_matches(board: &Board, width: usize, height: usize, is_vertical: bool) -> Vec<Match> {
    let mut matches = vec![];
    for i in 0..height {
        let mut matched: Vec<Point<usize>> = vec![];
        for j in 0..width {
            let point = if is_vertical {
                Point(i, j)
            } else {
                Point(j, i)
            };
            if let Some(color) = board.color_at_point(board.as_ref(), point) {
                if matched.is_empty()
                    || color == board.color_at_point(board.as_ref(), matched[0]).unwrap()
                {
                    matched.push(point)
                } else {
                    if matched.len() > 2 {
                        matches.push(Match::new(matched.clone()));
                    }
                    matched = vec![point];
                }
            }
        }
        if matched.len() > 2 {
            matches.push(Match::new(matched.clone()));
        }
    }
    return matches;
}
