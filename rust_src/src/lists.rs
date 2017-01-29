extern crate libc;

use std::os::raw::c_char;
use std::ptr;
use std::mem;

use lisp::{LispObject, LispType, Qnil, XTYPE, XUNTAG, wrong_type_argument};

#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list);
    list
}
