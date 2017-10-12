//! # Levenshtein distance
//!
//! A very simple crate to calculate the [Levenshtein edit distance][wikipedia] of two strings
//!
//! Should be fast and memory efficient by two vectors instead of a matrix
//!
//! ## References
//! [Wikipedia: Levenshtein distance][wikipedia]
//!
//! [wikipedia]: http://en.wikipedia.org/wiki/Levenshtein_distance

use std::mem::swap;
use std::cmp::min;


/// Calculates the Levenshtein distance between strings `s` and `t`.
///
/// Uses the two vector method instead of the matrix method
/// as defined [here](https://www.codeproject.com/Articles/13525/Fast-memory-efficient-Levenshtein-algorithm)
///
/// # Examples
///
/// ```rust
/// use levenshtein::distance;
///
/// assert_eq!(distance("kitten", "sitting"), 3);
/// ```
pub fn distance(s: &str, t: &str) -> usize {
    let m = s.len();
    let n = t.len();

    let s0: Vec<_> = s.chars().collect();
    let s1: Vec<_> = t.chars().collect();

    let mut v0: Vec<_> = (0..n + 1).collect();
    let mut v1 = vec![0; n + 1];

    for i in 0..m {
        v1[0] = i + 1;

        for j in 0..n {
            let substitution_cost;
            if s0[i] == s1[j] {
                substitution_cost = 0;
            } else {
                substitution_cost = 1;
            }
            v1[j + 1] = min(min(v1[j] + 1, v0[j + 1] + 1), v0[j] + substitution_cost);
        }
        swap(&mut v0, &mut v1);
    }
    return v0[n];
}

#[test]
fn deletion() {
    assert_eq!(distance("fool", "foo"), 1);
}

#[test]
fn deletion_2() {
    assert_eq!(distance("lore", "lo"), 2);
}

#[test]
fn insertion() {
    assert_eq!(distance("ball", "balls"), 1);
}

#[test]
fn insertion_2() {
    assert_eq!(distance("fo", "food"), 2);
}

#[test]
fn change() {
    assert_eq!(distance("yawn", "yawl"), 1);
}

#[test]
fn change_2() {
    assert_eq!(distance("yawn", "brawn"), 2);
}

#[test]
fn deletion_insertion() {
    assert_eq!(distance("flaw", "lawn"), 2);
}