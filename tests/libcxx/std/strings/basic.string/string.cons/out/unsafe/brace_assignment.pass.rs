extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0() -> bool {
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"hello world".as_ptr(),
            libcc2rs::c_str_len(c"hello world".as_ptr()) + 1,
        )
        .to_vec();
        unimplemented!();
        assert!(Vec::len(&s) <= 1);
        const _: () = assert!(true, "");;
    }
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"hello world".as_ptr(),
            libcc2rs::c_str_len(c"hello world".as_ptr()) + 1,
        )
        .to_vec();
        s = std::slice::from_raw_parts(c"abc".as_ptr(), 2_usize as usize)
            .to_vec()
            .iter()
            .copied()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        assert!(
            s == std::slice::from_raw_parts(
                c"ab".as_ptr(),
                libcc2rs::c_str_len(c"ab".as_ptr()) + 1
            )
            .to_vec()
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"hello world".as_ptr(),
            libcc2rs::c_str_len(c"hello world".as_ptr()) + 1,
        )
        .to_vec();
        s = std::slice::from_raw_parts(
            c"It\'sALongString!NoSSO!qwertyuiop".as_ptr(),
            30_usize as usize,
        )
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
        assert!(
            s == std::slice::from_raw_parts(
                c"It\'sALongString!NoSSO!qwertyui".as_ptr(),
                libcc2rs::c_str_len(c"It\'sALongString!NoSSO!qwertyui".as_ptr()) + 1
            )
            .to_vec()
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            libcc2rs::c_str_len(
                c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            ) + 1,
        )
        .to_vec();
        s = std::slice::from_raw_parts(
            c"It\'sALongString!NoSSO!qwertyuiop".as_ptr(),
            30_usize as usize,
        )
        .to_vec()
        .iter()
        .copied()
        .chain(std::iter::once((0 as libc::c_char)))
        .collect();
        assert!(
            s == std::slice::from_raw_parts(
                c"It\'sALongString!NoSSO!qwertyui".as_ptr(),
                libcc2rs::c_str_len(c"It\'sALongString!NoSSO!qwertyui".as_ptr()) + 1
            )
            .to_vec()
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            libcc2rs::c_str_len(
                c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            ) + 1,
        )
        .to_vec();
        s = std::slice::from_raw_parts(c"abc".as_ptr(), 2_usize as usize)
            .to_vec()
            .iter()
            .copied()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        assert!(
            s == std::slice::from_raw_parts(
                c"ab".as_ptr(),
                libcc2rs::c_str_len(c"ab".as_ptr()) + 1
            )
            .to_vec()
        );
        const _: () = assert!(true, "");;
    }
    {
        let mut s: Vec<libc::c_char> = std::slice::from_raw_parts(
            c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            libcc2rs::c_str_len(
                c"Hello world! Hello world! Hello world! Hello world! Hello world!".as_ptr(),
            ) + 1,
        )
        .to_vec();
        s = std::slice::from_raw_parts(c"abc".as_ptr(), 0_usize as usize)
            .to_vec()
            .iter()
            .copied()
            .chain(std::iter::once((0 as libc::c_char)))
            .collect();
        assert!(
            s == std::slice::from_raw_parts(c"".as_ptr(), libcc2rs::c_str_len(c"".as_ptr()) + 1)
                .to_vec()
        );
        const _: () = assert!(true, "");;
    }
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
