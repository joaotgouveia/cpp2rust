extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Fun {}
impl Fun {
    pub unsafe fn operator_call_1_const_char__ref_arr6arr_int_ref(
        &self,
        _a0: *const [libc::c_char; 6],
        _a1: *mut i32,
    ) {
        const _: () = assert!(true, "std::is_same<U, int &>::value");;
    }
}
impl Callable2<*const [libc::c_char; 6], *mut i32, ()> for Fun {
    fn call(&self, a1: *const [libc::c_char; 6], a2: *mut i32) -> () {
        unsafe { Fun::operator_call_1_const_char__ref_arr6arr_int_ref(self, a1, a2) }
    }
}
pub unsafe fn test_0() -> bool {
    (unsafe { unimplemented!()(&std::mem::transmute(*b"hello\0")) });
    return true;
}
pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
