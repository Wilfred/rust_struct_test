extern crate libc;

pub type EmacsInt = libc::c_int;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);
