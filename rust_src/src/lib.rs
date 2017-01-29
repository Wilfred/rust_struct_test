extern crate libc;

pub type EmacsInt = libc::c_int;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(pub EmacsInt);


#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list.0);
    list
}

