// use crate::prelude::*;

pub fn cartesian_to_polar(x: f32, y: f32) -> (f32, f32) { ((x * x + y * y).sqrt(), y.atan2(x)) }

pub fn polar_to_cartesian(r: f32, t: f32) -> (f32, f32) { (r * t.cos(), r * t.sin()) }
