extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, CppFromUnsafe, Default)]
#[cpp_from(libc::c_char, new, value)]
#[cpp_into(libc::c_char, to_libcc_char)]
pub struct Charlike_Holder_Incomplete__ {
    pub ch_: libc::c_char,
}
impl Charlike_Holder_Incomplete__ {
    pub unsafe fn new(mut ch: libc::c_char) -> Self {
        let mut this = Self { ch_: ch };
        this
    }
    pub unsafe fn to_libcc_char(&self) -> libc::c_char {
        return self.ch_;
    }
}
pub unsafe fn test_string_0() {
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    let mut a: [Charlike_Holder_Incomplete__; 4] = [
        Charlike_Holder_Incomplete__::new({ ('m' as libc::c_char) }),
        Charlike_Holder_Incomplete__::new({ ('a' as libc::c_char) }),
        Charlike_Holder_Incomplete__::new({ ('h' as libc::c_char) }),
        Charlike_Holder_Incomplete__::new({ ('i' as libc::c_char) }),
    ];
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    assert!(
        s == std::slice::from_raw_parts(
            c"mahimahi".as_ptr(),
            libcc2rs::c_str_len(c"mahimahi".as_ptr()) + 1
        )
        .to_vec()
    );
}
pub unsafe fn test_1() -> bool {
    (unsafe { test_string_0() });
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
    (unsafe { test_1() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
