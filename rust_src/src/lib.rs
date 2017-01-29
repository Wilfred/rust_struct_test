extern crate libc;

#[repr(C)]
pub struct LispObject(libc::c_int);

#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list.0);
    list
}

