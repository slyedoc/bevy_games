// From https://github.com/mitchmindtree/utils-rs/blob/master/src/math.rs
// and 

///
///  jmath.rs
///
///  Created by Mitchell Nordine at 04:23AM on May 30, 2014.
///
///

// use num::{Float, NumCast};
// use num::PrimInt as Int;
// use std::mem;

// /// Clamp a value to a range.
// #[inline]
// pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
//     if val < min { min } else { if val > max { max } else { val } }
// }

// /// Models the CPP fmod function.
// #[inline]
// pub fn fmod<F: Float>(numer: F, denom: F) -> F {
//     let rquot: F = (numer / denom).floor();
//     numer - rquot * denom
// }

// /// Check if value is in range.
// #[inline]
// pub fn in_range<T: Ord>(val: T, min: T, max: T) -> bool {
//     val >= min && val <= max
// }

/// Interpolate from start to stop 'amt' amount.
#[inline]
pub fn lerp<T: std::marker::Copy + std::ops::Sub<Output=T> + std::ops::Mul<Output=T> + std::ops::Add<Output=T> >(start: T, stop: T, amt: T) -> T {
    start + (stop - start) * amt
}

/// Map a value from a given range to a new given range.
#[inline]
pub fn map_range<X>(val: X, in_min: X, in_max: X, out_min: X, out_max: X) -> X where
    X: std::marker::Copy + std::ops::Sub<Output=X> + std::ops::Mul<Output=X> + std::ops::Div<Output=X> + std::ops::Add<Output=X>,
{

    (val - in_min) / (in_max - in_min) * (out_max - out_min) + out_min
}
