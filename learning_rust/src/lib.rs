/*
   Publishing your Crate
*/

//! # Basic math crate
//! This is a collection of some generally used math functions
//!
/// Compute a square of an input number
///
/// # Examples
/// # Test
/// ```
/// let n = 5;
/// let answer = learning_rust::square(n);
/// assert_eq!(25, answer);
/// ```
/// # limitations
///
/// # Some other section
///
pub fn square(num: i32) -> i32 {
    num * num
}

/// Compute a cube of an input number
///
/// # Examples
/// # Test
/// ```
/// let n = 5;
/// let answer = learning_rust::cube(n);
/// assert_eq!(125, answer);
/// ```
/// # limitations
///
/// # Some other section
///
pub fn cube(num: i32) -> i32 {
    num * num * num
}
