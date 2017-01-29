#![allow(non_upper_case_globals)]

extern crate libc;

use std::os::raw::c_char;
use std::mem;
use std::fmt::{Debug, Formatter, Error};

pub type EmacsInt = libc::c_int;
pub type EmacsUint = libc::c_uint;
pub const EMACS_INT_MAX: EmacsInt = 0x7FFFFFFF_i32;
pub const EMACS_INT_SIZE: EmacsInt = 4;
pub const GCTYPEBITS: EmacsInt = 3;
pub const USE_LSB_TAG: bool = true;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);

pub const Qnil: LispObject = LispObject(0);

impl LispObject {
    #[inline]
    pub fn to_raw(self) -> EmacsInt {
        self.0
    }
}

// Number of bits in a Lisp_Object tag.
#[allow(dead_code)]
const VALBITS: EmacsInt = EMACS_INT_SIZE * 8 - GCTYPEBITS;

const INTTYPEBITS: EmacsInt = GCTYPEBITS - 1;

const VAL_MAX: EmacsInt = EMACS_INT_MAX >> (GCTYPEBITS - 1);

const VALMASK: EmacsInt = [VAL_MAX, -(1 << GCTYPEBITS)][USE_LSB_TAG as usize];

/// Bit pattern used in the least significant bits of a lisp object,
/// to denote its type.
#[repr(u8)]
#[derive(PartialEq, Eq)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug)]
pub enum LispType {
    Lisp_Symbol = 0,
    Lisp_Misc = 1,
    Lisp_Int0 = 2,
    Lisp_Int1 = 3 + (USE_LSB_TAG as usize as u8) * 3, // 3 | 6
    Lisp_String = 4,
    Lisp_Vectorlike = 5,
    Lisp_Cons = 6 - (USE_LSB_TAG as usize as u8) * 3, // 6 | 3
    Lisp_Float = 7,
}
