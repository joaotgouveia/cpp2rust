extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn make_implicit_0() -> () {
    return unimplemented!();
}
pub unsafe fn make_implicit_1() -> () {
    return unimplemented!();
}
pub unsafe fn test_convertible_imp_2(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn test_convertible_imp_3(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn test_convertible_imp_4(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_5(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_6(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_imp_7(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_8() -> bool {
    return (unsafe { test_convertible_imp_2(0) });
}
pub unsafe fn test_convertible_9() -> bool {
    return (unsafe { test_convertible_imp_4(0_i64) });
}
pub unsafe fn test_convertible_10() -> bool {
    return (unsafe { test_convertible_imp_5(0_i64) });
}
pub unsafe fn test_convertible_11() -> bool {
    return (unsafe { test_convertible_imp_3(0) });
}
pub unsafe fn test_convertible_12() -> bool {
    return (unsafe { test_convertible_imp_6(0_i64) });
}
pub unsafe fn test_convertible_13() -> bool {
    return (unsafe { test_convertible_imp_7(0_i64) });
}
pub unsafe fn test_implicit_14() {
    const _: () = assert!(true, "test_convertible<D>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<D, T>()");;
    const _: () = assert!(true, "!test_convertible<D, T, double>()");;
}
pub unsafe fn test_implicit_15() {
    const _: () = assert!(true, "test_convertible<D>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<D, T>()");;
    const _: () = assert!(true, "!test_convertible<D, T, double>()");;
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
        let mut d: () = unimplemented!();
        assert!(((unimplemented!()) == (1)));
        assert!(((unimplemented!()) == (0.5_f64)));
    }
    {
        let mut d: () = unimplemented!();
        assert!(((unimplemented!()) == (3)));
        assert!(((unimplemented!()) == (0.5_f64)));
    }
    {
        let mut d: () = unimplemented!();
        assert!(((unimplemented!()) == (3)));
        assert!(((unimplemented!()) == (0.75_f64)));
    }
    (unsafe { test_implicit_14() });
    (unsafe { test_implicit_15() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
