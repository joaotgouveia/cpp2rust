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
#[cpp_from(libc::c_char, new_1, value)]
pub struct char_ascii {
    pub char_: libc::c_char,
}
impl char_ascii {
    pub unsafe fn new_1(mut ch: libc::c_char) -> Self {
        let mut this = Self { char_: ch };
        assert!(((ch as i32) <= (127)));
        this
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
    let mut str: Vec<char_ascii> = vec![char_ascii::default()];
    let mut ch: char_ascii = char_ascii::new_1({ ('A' as libc::c_char) });
    unimplemented!();
    assert!(((str.len() - 1) == (1_usize)));
    assert!(
        (((*(&mut str).as_mut_ptr().add(0_usize as usize)).char_ as i32)
            == (('A' as libc::c_char) as i32))
    );
    const _: () = assert!(true, "");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
