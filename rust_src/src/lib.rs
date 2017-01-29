extern crate libc;

#[repr(C)]
pub struct LispObject(libc::c_int);

#[no_mangle]
pub extern "C" fn fcdr(list: LispObject) -> LispObject {
    println!("fcdr: {:?}", list.0);
    list
}

