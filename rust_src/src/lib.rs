#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

extern crate libc;

mod lisp;
mod lists;

// Widely used in the C codebase.
pub use lists::Fcar;
pub use lists::Fcdr;
