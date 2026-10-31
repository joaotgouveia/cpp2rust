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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
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
        let mut i: u32 = 16909060_u32;
        let mut c: [libc::c_char; 4] = [(0 as libc::c_char); 4];
        const _: () = assert!(
            ((::std::mem::size_of::<u32>()) == (::std::mem::size_of::<[libc::c_char; 4]>())),
            "sizeof(i) == sizeof(c)"
        );;
        {
            if ::std::mem::size_of::<[libc::c_char; 4]>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    ((&mut i as *mut u32) as *const ::libc::c_void),
                    (c.as_mut_ptr() as *mut ::libc::c_void),
                    ::std::mem::size_of::<[libc::c_char; 4]>() as usize,
                )
            }
            (c.as_mut_ptr() as *mut ::libc::c_void)
        };
        assert!(
            ((((c[(0) as usize] as i32) == (1)) as i32)
                == (((unimplemented!()) == (unimplemented!())) as i32))
        );
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
