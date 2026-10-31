extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { i16::MIN } else { i16::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_1(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { u16::MIN } else { u16::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_2(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { i32::MIN } else { i32::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_3(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { u32::MIN } else { u32::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_4(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { i64::MIN } else { i64::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_5(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { u64::MIN } else { u64::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_6(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() { i64::MIN } else { i64::MAX }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
}
pub unsafe fn test_7(mut fmt: (), mut expected: *const libc::c_char) {
    let mut ss: () = unimplemented!();
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (ss as ::std::fs::File);
            let __s: *mut ::std::fs::File = __r;
            (libcc2rs::showbase_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(
                __s,
            )
        });
        let __s: *mut ::std::fs::File = __r;
        libcc2rs::write_int(
            &mut *__s,
            (if unimplemented!() {
                unimplemented!()
            } else {
                unimplemented!()
            }),
        );
        __s
    };
    assert!(
        unimplemented!()
            == std::slice::from_raw_parts(expected, libcc2rs::c_str_len(expected) + 1).to_vec()
    );
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
    let o: () = unimplemented!();
    let d: () = unimplemented!();
    let x: () = unimplemented!();
    (unsafe { test_0(o, c"0100000".as_ptr()) });
    (unsafe { test_0(d, c"-32768".as_ptr()) });
    (unsafe { test_0(x, c"0x8000".as_ptr()) });
    (unsafe { test_1(o, c"0177777".as_ptr()) });
    (unsafe { test_1(d, c"65535".as_ptr()) });
    (unsafe { test_1(x, c"0xffff".as_ptr()) });
    (unsafe { test_2(o, c"020000000000".as_ptr()) });
    (unsafe { test_2(d, c"-2147483648".as_ptr()) });
    (unsafe { test_2(x, c"0x80000000".as_ptr()) });
    (unsafe { test_3(o, c"037777777777".as_ptr()) });
    (unsafe { test_3(d, c"4294967295".as_ptr()) });
    (unsafe { test_3(x, c"0xffffffff".as_ptr()) });
    let long_is_32: bool = unimplemented!();
    let long_is_64: bool = unimplemented!();
    let long_long_is_64: bool = unimplemented!();
    {
        {
            (unsafe { test_4(o, c"01000000000000000000000".as_ptr()) });
            (unsafe { test_4(d, c"-9223372036854775808".as_ptr()) });
            (unsafe { test_4(x, c"0x8000000000000000".as_ptr()) });
            (unsafe { test_5(o, c"01777777777777777777777".as_ptr()) });
            (unsafe { test_5(d, c"18446744073709551615".as_ptr()) });
            (unsafe { test_5(x, c"0xffffffffffffffff".as_ptr()) });
        }
    }
    {
        (unsafe { test_6(o, c"01000000000000000000000".as_ptr()) });
        (unsafe { test_6(d, c"-9223372036854775808".as_ptr()) });
        (unsafe { test_6(x, c"0x8000000000000000".as_ptr()) });
        (unsafe { test_7(o, c"01777777777777777777777".as_ptr()) });
        (unsafe { test_7(d, c"18446744073709551615".as_ptr()) });
        (unsafe { test_7(x, c"0xffffffffffffffff".as_ptr()) });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
