//! Simple, reusable and optimized XOR ciphers in Rust.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#![cfg_attr(not(feature = "std"), no_std)]

pub mod in_place;
