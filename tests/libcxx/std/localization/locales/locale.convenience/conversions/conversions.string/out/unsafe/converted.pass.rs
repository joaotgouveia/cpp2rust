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
pub struct TestHelper_wchar_t__4_ {}
impl TestHelper_wchar_t__4_ {
    pub unsafe fn test() {
        const _: () = assert!(true, "(std::is_same<CharT, wchar_t>::value)");;
        {
            let mut myconv: () = unimplemented!();
            assert!(((unimplemented!()) == (0_usize)));
            let mut bs: Vec<libc::c_char> = unimplemented!();
            assert!(((unimplemented!()) == (1_usize)));
            bs = unimplemented!();
            assert!(((unimplemented!()) == (2_usize)));
            let mut ws: Vec<i32> = unimplemented!();
            assert!(((unimplemented!()) == (4_usize)));
        }
    }
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
    (unsafe { TestHelper_wchar_t__4_::test() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
