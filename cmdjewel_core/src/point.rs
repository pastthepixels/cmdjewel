use std::ops::{Add, Sub};

/// Specifies a 2D x,y point
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Point<T>(pub T, pub T)
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq;

impl<T> Add<Point<T>> for Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T> Sub<Point<T>> for Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T> Point<T>
where
    T: Add<Output = T> + Sub<Output = T> + PartialEq,
{
    pub fn distance_to(from: Point<f32>, to: Point<f32>) -> f32 {
        let difference = to - from;
        f32::sqrt(difference.0.powi(2) + difference.1.powi(2))
    }
    // Gets a list of all adjacent points for a usize point
    pub fn get_adjacent_usize(point: Point<usize>) -> Vec<Point<usize>> {
        let mut points_valid: Vec<Point<usize>> = Vec::new();
        let point = Point(point.0 as i32, point.1 as i32);
        [
            Point(point.0, point.1 + 1),
            Point(point.0, point.1 - 1),
            Point(point.0 + 1, point.1),
            Point(point.0 + 1, point.1 + 1),
            Point(point.0 + 1, point.1 - 1),
            Point(point.0 - 1, point.1),
            Point(point.0 - 1, point.1 + 1),
            Point(point.0 - 1, point.1 - 1),
        ]
        .iter()
        .for_each(|adjacent| {
            if adjacent.0 >= 0 && adjacent.1 >= 0 {
                points_valid.push(Point(adjacent.0 as usize, adjacent.1 as usize));
            }
        });
        points_valid
    }
    // Gets a list of all points that touch a point to one edge for a usize point
    pub fn get_edge_usize(point: Point<usize>) -> Vec<Point<usize>> {
        let mut points_valid: Vec<Point<usize>> = Vec::new();
        let point = Point(point.0 as i32, point.1 as i32);
        [
            Point(point.0, point.1 + 1),
            Point(point.0, point.1 - 1),
            Point(point.0 + 1, point.1),
            Point(point.0 - 1, point.1),
        ]
        .iter()
        .for_each(|adjacent| {
            if adjacent.0 >= 0 && adjacent.1 >= 0 {
                points_valid.push(Point(adjacent.0 as usize, adjacent.1 as usize));
            }
        });
        points_valid
    }
}

/// Specifies adjacent directions
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}
