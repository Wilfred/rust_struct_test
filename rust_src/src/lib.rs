extern crate libc;

mod lisp;

use lisp::{LispObject};

#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list);
    list
}

