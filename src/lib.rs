#![cfg_attr(not(feature = "std"), no_std)]
pub mod desc;
pub mod parse;
pub mod srcinfo;
pub mod value;

#[cfg(feature = "std")]
pub use indexmap;
#[cfg(feature = "parking_lot")]
pub use parking_lot;
