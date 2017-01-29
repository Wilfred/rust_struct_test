extern crate libc;

#[repr(C)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct LispObject(libc::c_int);


#[no_mangle]
pub extern "C" fn Fcdr(list: LispObject) -> LispObject {
    println!("Fcdr: {:?}", list.0);
    list
}

