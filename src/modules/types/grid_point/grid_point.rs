use std::ops::Sub;

use crate::prelude::*;

/// A trait for types representing a 2d Point.
pub trait GridPoint: Clone + Copy {
    #[allow(clippy::new_ret_no_self)]
    /// Construct a IVec2
    fn new(x: i32, y: i32) -> IVec2 { IVec2::new(x, y) }

    /// Returns x position.
    fn x(&self) -> i32;

    /// Returns y position.
    fn y(&self) -> i32;

    /// Returns the grid point offset by the given amount.
    fn offset(&self, xy: impl GridPoint) -> IVec2 { self.add(xy) }

    /// Convert point to `IVec2` (i32).
    #[inline]
    fn as_ivec2(&self) -> IVec2 { IVec2::new(self.x(), self.y()) }

    /// Convert point to `UVec2` (u32).
    #[inline]
    fn as_uvec2(&self) -> UVec2 { self.as_ivec2().as_uvec2() }

    /// Convert point to `Vec2` (f32).
    #[inline]
    fn as_vec2(&self) -> Vec2 { self.as_ivec2().as_vec2() }

    /// Convert point to `[i32; 2]`.
    #[inline]
    fn as_array(&self) -> [i32; 2] { self.as_ivec2().to_array() }

    /// Get the point's corresponding 1d index.
    #[inline(always)]
    fn as_index_unchecked<I: TryInto<usize>>(&self, width: I) -> usize {
        let width = width.try_into().unwrap_or_else(|_v| panic!("Something went wrong!"));
        self.y() as usize * width + self.x() as usize
    }

    #[inline(always)]
    fn as_index(&self, size: impl Size2d) -> Option<usize> {
        if self.is_valid(size) {
            Some(self.as_index_unchecked(size.width()))
        } else {
            None
        }
    }

    /// Returns true if the point is valid for the given size.
    #[inline]
    fn is_valid(&self, size: impl Size2d) -> bool {
        let x = self.x();
        let y = self.y();
        x >= 0 && y >= 0 && (x as u32) < size.width() && (y as u32) < size.height()
    }

    ////////////////
    //   Math     //
    ////////////////
    /// Adds two points together.
    #[inline]
    fn add(&self, other: impl GridPoint) -> IVec2 { IVec2::new(self.x() + other.x(), self.y() + other.y()) }

    /// Returns distance from another `GridPoint`.
    #[inline]
    fn distance(&self, other: impl GridPoint) -> f32 { self.as_vec2().distance(other.as_vec2()) }

    /// The [taxicab distance](https://en.wikipedia.org/wiki/Taxicab_geometry)
    /// between two grid points.
    #[inline]
    fn taxi_dist(self, other: impl GridPoint) -> i32 {
        // optimized as integir math for speed as GridPoint is used everywhere.
        self.as_ivec2().sub(other.as_ivec2()).abs().max_element()
    }

    /// Linearly interpolate between points a and b by the amount t.
    #[inline]
    fn lerp(self, other: impl GridPoint, t: f32) -> IVec2 {
        self.as_vec2().lerp(other.as_vec2(), t).as_ivec2()
    }

    ////////////////
    //  Geometry  //
    ////////////////
    #[inline]
    fn from_angle(center: impl GridPoint, distance: f32, degrees: f32) -> IVec2 {
        let rads = degrees.to_radians();
        let x = (distance * rads.cos()).floor() as i32; // .round() ??
        let y = (distance * rads.sin()).floor() as i32;
        IVec2::new(center.x() + x, center.y() + y)
    }

    #[inline]
    fn angle_to(&self, point: impl GridPoint) -> f32 {
        let x = (point.x() - self.x()) as f32;
        let y = (point.y() - self.y()) as f32;
        y.atan2(x).to_degrees()
    }

    #[inline]
    fn mid_point(&self, point: impl GridPoint) -> IVec2 {
        IVec2 {
            x: (self.x() + point.x()) / 2,
            y: (self.y() + point.y()) / 2,
        }
    }

    /// Returns the `Cross Product` between two points.
    #[inline]
    fn cross_product(&self, point: impl GridPoint) -> i32 { self.x() * point.y() - self.y() * point.x() }

    /// Returns the `Dot Product` between two points.
    #[inline]
    fn dot_product(&self, point: impl GridPoint) -> i32 { self.x() * point.x() + self.y() * point.y() }

    /// Returns the grid point the given number of spaces above this one.
    #[inline]
    fn up(&self, amount: i32) -> IVec2 { IVec2::new(self.x(), self.y() + amount) }

    /// Returns the grid point the given number of spaces below this one.
    #[inline]
    fn down(&self, amount: i32) -> IVec2 { IVec2::new(self.x(), self.y() - amount) }

    /// Returns the grid point the given number of spaces to the right of
    /// this one.
    #[inline]
    fn right(&self, amount: i32) -> IVec2 { IVec2::new(self.x() + amount, self.y()) }

    /// Returns the grid point the given number of spaces to the left of
    /// this one.
    #[inline]
    fn left(&self, amount: i32) -> IVec2 { IVec2::new(self.x() - amount, self.y()) }

    ////////////////
    //  Iterator  //
    ////////////////
    /// Returns an iterator over the 8 points adjacent to this one. (N, NE, E, SE, S, SW, W,
    /// NW)
    #[inline]
    fn neighbors_all(&self) -> AdjIterator { AdjIterator::new(*self, GridDirection::all().collect()) }

    /// Returns an iterator over the 4 points cardinal - adjacent to this one. (N, E, S, W)
    #[inline]
    fn neighbors_cardinal(&self) -> AdjIterator {
        AdjIterator::new(*self, CardinalDirection::all_directions().collect())
    }

    /// Returns an iterator over the 4 points ordinal - adjacent to this one. (NE, SE, SW,
    /// NW)
    #[inline]
    fn neighbors_ordinal(&self) -> AdjIterator {
        AdjIterator::new(*self, OrdinalDirection::all_directions().collect())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    #[test]
    fn taxi() {
        let a = (10, 10);
        let b = (20, 20);
        let dist = GridPoint::taxi_dist(a, b);
        assert_eq!(dist, 20);
    }

    #[test]
    fn adj() {
        let points: Vec<IVec2> = (10, 10).neighbors_cardinal().collect();
        assert!(points.contains(&IVec2::new(10, 9)));
        assert!(points.contains(&IVec2::new(9, 10)));
        assert!(points.contains(&IVec2::new(11, 10)));
        assert!(points.contains(&IVec2::new(10, 11)));
        let points: Vec<IVec2> = (10, 10).neighbors_all().collect();
        assert!(points.contains(&IVec2::new(10, 9)));
        assert!(points.contains(&IVec2::new(9, 10)));
        assert!(points.contains(&IVec2::new(11, 10)));
        assert!(points.contains(&IVec2::new(10, 11)));
        assert!(points.contains(&IVec2::new(11, 11)));
        assert!(points.contains(&IVec2::new(9, 9)));
        assert!(points.contains(&IVec2::new(11, 9)));
        assert!(points.contains(&IVec2::new(9, 11)));
    }
}
