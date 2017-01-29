extern crate libc;

use std::os::raw::c_char;
use std::ptr;
use std::mem;

use lisp::{CHECK_TYPE, LispObject, LispSubr, LispType, Qnil, XTYPE, XUNTAG, wrong_type_argument};

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

/// Set the car of a cons cell.
fn XSETCAR(c: LispObject, n: LispObject) {
    let cons_cell = XCONS(c);
    unsafe {
        (*cons_cell).car = n;
    }
}

/// Set the cdr of a cons cell.
fn XSETCDR(c: LispObject, n: LispObject) {
    let cons_cell = XCONS(c);
    unsafe {
        (*cons_cell).cdr = n;
    }
}

/// Is `object` nil?
pub fn NILP(object: LispObject) -> bool {
    object == Qnil
}

unsafe fn XCAR(object: LispObject) -> LispObject {
    (*XCONS(object)).car
}

unsafe fn XCDR(object: LispObject) -> LispObject {
    (*XCONS(object)).cdr
}

/// Take the car/cdr of a cons cell, or signal an error if it's a
/// different type.
///
/// # Porting Notes
///
/// This is equivalent to `CAR`/`CDR` in C code.
fn car(object: LispObject) -> LispObject {
    if CONSP(object) {
        unsafe { XCAR(object) }
    } else if NILP(object) {
        Qnil
    } else {
        unsafe { wrong_type_argument(Qnil, object) }
    }
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
pub extern "C" fn Fcar(list: LispObject) -> LispObject {
    println!("Fcar: {:?}", list);
    car(list)
}

defun!("car",
       Fcar,
       Scar,
       1,
       1,
       ptr::null(),
       "Return the car of LIST.  If arg is nil, return nil.
Error if arg is not nil and not a \
        cons cell.  See also `car-safe'.

See Info node `(elisp)Cons Cells' for a discussion of \
        related basic
Lisp concepts such as car, cdr, cons cell and list.

(fn LIST)");

#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list);
    cdr(list)
}

defun!("cdr",
       Fcdr,
       Scdr,
       1,
       1,
       ptr::null(),
       "Return the cdr of LIST.  If arg is nil, return nil.
Error if arg is not nil and not a \
        cons cell.  See also `cdr-safe'.

See Info node `(elisp)Cons Cells' for a discussion of \
        related basic
Lisp concepts such as cdr, car, cons cell and list.

(fn LIST)");

