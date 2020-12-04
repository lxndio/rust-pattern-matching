// Copyright 2020 Alexander Korn
//
// Licensed under the MIT license

//! A library containing pattern matching and full-text index algorithms.
//!
//! # How to Start
//!
//! As you are looking at this documentation, I assume that you'd like to
//! match a pattern on a text. This library contains different algorithms
//! doing exactly that. Which algorithm you should use, however, depends
//! strongly on your use case.
//!
//! If you don't already know, which algorithms would suite your needs,
//! you should click through the algorithms listed below and have a look
//! at the *When to Use It* parts of their descriptions.
//!
//! # Algorithms
//!
//! Currently, the following algorithms are implemented:
//!
//! - [BNDM](single_pattern::bndm)
//! - [Horspool](single_pattern::horspool)
//! - [Naive](single_pattern::naive)
//! - [Shift-And](single_pattern::shift_and)

pub mod single_pattern;
