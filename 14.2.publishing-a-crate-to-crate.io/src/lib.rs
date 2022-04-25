//! Hello docs
//! # My Crate
//!
//! `publish_test_with_rust_book` is a collection of utilities to make performing certain
//! calculations more convenient

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = publish::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub use self::{
    kinds::{PrimaryColor, SecondaryColor},
    utils::mix,
};

pub mod kinds {
    //! # Art
    //!
    //! A library for modeling artistic concepts

    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
