extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_convertible_imp_0(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn test_convertible_imp_1(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_3() -> bool {
    return (unsafe { test_convertible_imp_0(0) });
}
pub unsafe fn test_convertible_4() -> bool {
    return (unsafe { test_convertible_imp_1(0_i64) });
}
pub unsafe fn test_convertible_5() -> bool {
    return (unsafe { test_convertible_imp_2(0_i64) });
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
    {
        const _: () = assert!(
            true,
            "std::is_nothrow_default_constructible<file_status>::value"
        );;
        const _: () = assert!(true, "test_convertible<file_status>()");;
        let f: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        const _: () = assert!(
            true,
            "std::is_nothrow_constructible<file_status, file_type>::value"
        );;
        const _: () = assert!(true, "!test_convertible<file_status, file_type>()");;
        let f: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    {
        const _: () = assert!(
            true,
            "std::is_nothrow_constructible<file_status, file_type, perms>::value"
        );;
        const _: () = assert!(true, "!test_convertible<file_status, file_type, perms>()");;
        let f: () = unimplemented!();
        assert!(((unimplemented!()) == (unimplemented!())));
        assert!(((unimplemented!()) == (unimplemented!())));
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
