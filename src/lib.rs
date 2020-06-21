//! # Rearranging pdf pages
//!
//! There are two separate function:
//!
//! 1.  calculating the new order for the pages, and
//! 2.  rearranging the pages themselves.

/// For testing, quickcheck is invaluable.
#[cfg(test)]
#[macro_use]
extern crate quickcheck;

/// Read and write pdf files with `lopdf`.
/// The interface is pleasant, but there are some lacking features (at least at time of writing)
/// which are fixed in the `extra` modules.
extern crate lopdf;

/// Numerical processing.
extern crate num;
#[macro_use]
extern crate num_derive;

/// Calculate the new page order.
pub mod calculate;
pub use calculate::*;

/// Actually rearrange the pages.
pub mod rearrange;
pub use rearrange::*;

/// The extra modules supply functionality that is not specific to pdf processing.
mod extra {
    /// Generate error values.
    pub mod error;
}
