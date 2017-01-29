#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

#![cfg_attr(feature = "strict", deny(warnings))]

// Wilfred/remacs#38 : Need to override the allocator for legacy unexec support on Mac.
#[cfg(target_os = "macos")]
extern crate alloc_unexecmacosx;

#[macro_use]
extern crate lazy_static;

extern crate libc;

mod lisp;
mod lists;

// Widely used in the C codebase.
pub use lists::Fcar;
pub use lists::Fcdr;
