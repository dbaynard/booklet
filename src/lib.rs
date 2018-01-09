#[cfg(test)]
#[macro_use]
extern crate quickcheck;

extern crate lopdf;

extern crate num;
#[macro_use]
extern crate num_derive;

pub mod calculate;
pub use calculate::*;

pub mod rearrange;
pub use rearrange::*;

mod extra {
    pub mod error;

    pub mod lopdf;
}
