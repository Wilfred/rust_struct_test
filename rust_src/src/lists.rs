extern crate libc;

use std::os::raw::c_char;
use std::ptr;
use std::mem;

use lisp::{LispObject, LispSubr, LispType, Qnil, XTYPE, XUNTAG, wrong_type_argument};

pub fn CONSP(x: LispObject) -> bool {
    XTYPE(x) == LispType::Lisp_Cons
}

/// Represents a cons cell, or GC bookkeeping for cons cells.
///
/// A cons cell is pair of two pointers, used to build linked lists in
/// lisp.
///
/// # C Porting Notes
///
/// The equivalent C struct is `Lisp_Cons`. Note that the second field
/// may be used as the cdr or GC bookkeeping.
// TODO: this should be aligned to 8 bytes.
#[repr(C)]
#[allow(unused_variables)]
struct LispCons {
    /// Car of this cons cell.
    car: LispObject,
    /// Cdr of this cons cell, or the chain used for the free list.
    cdr: LispObject,
}

// alloc.c uses a union for `Lisp_Cons`, which we emulate with an
// opaque struct.
#[repr(C)]
#[allow(dead_code)]
pub struct LispConsChain {
    chain: *const LispCons,
}

/// Extract the LispCons data from an elisp value.
fn XCONS(a: LispObject) -> *mut LispCons {
    debug_assert!(CONSP(a));
    unsafe { mem::transmute(XUNTAG(a, LispType::Lisp_Cons)) }
}

/// Is `object` nil?
pub fn NILP(object: LispObject) -> bool {
    object == Qnil
}

unsafe fn XCDR(object: LispObject) -> LispObject {
    (*XCONS(object)).cdr
}

fn cdr(object: LispObject) -> LispObject {
    if CONSP(object) {
        unsafe { XCDR(object) }
    } else if NILP(object) {
        Qnil
    } else {
        unsafe { wrong_type_argument(Qnil, object) }
    }
}

#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list);
    cdr(list)
}
