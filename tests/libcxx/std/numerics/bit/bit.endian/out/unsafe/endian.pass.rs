extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;

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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    const _: () = assert!(true, "std::is_enum<std::endian>::value");;
    const _: () = assert!(true, "!std::is_convertible<std::endian, UT>::value");;
    const _: () = assert!(true, "std::endian::little == std::endian::little");;
    const _: () = assert!(true, "std::endian::big    == std::endian::big");;
    const _: () = assert!(true, "std::endian::native == std::endian::native");;
    const _: () = assert!(true, "std::endian::little != std::endian::big");;
    const _: () = assert!(
        true,
        "std::endian::native == std::endian::little ||\n                   std::endian::native == std::endian::big"
    );;
    {
        let mut i: () = 16909060_();
        let mut c: [(); 4] = [unimplemented!(); 4];
        const _: () = assert!(
            ((::std::mem::size_of::<()>()) == (::std::mem::size_of::<[(); 4]>())),
            "sizeof(i) == sizeof(c)"
        );;
        unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
