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
pub unsafe fn test_convertible_imp_1(mut _a0: i32) -> bool {
    return true;
}
pub unsafe fn test_convertible_imp_2(mut _a0: i64) -> bool {
    return false;
}
pub unsafe fn test_convertible_3() -> bool {
    return (unsafe { test_convertible_imp_1(0) });
}
pub unsafe fn test_convertible_4() -> bool {
    return (unsafe { test_convertible_imp_2(0_i64) });
}
pub unsafe fn to_string_5(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_6(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_7(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_8(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_9(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_10(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_11(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_12(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_13(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_14(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_15(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_16(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_17(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_18(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_19(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_20(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_21(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_22(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_23(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_24(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_25(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_26(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_27(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn to_string_28(e: *const ()) -> Vec<libc::c_char> {
    let mut os: () = unimplemented!();
    unimplemented!();
    return unimplemented!();
}
pub unsafe fn test1_29() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_5(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_6(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_7(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test1_30() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_8(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_9(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_10(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test1_31() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_11(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_12(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_13(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test1_32() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_14(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_15(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_16(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test2_33() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_5(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_6(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_7(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test2_34() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_8(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_9(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_10(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test2_35() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_11(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_12(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_13(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test2_36() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_14(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_15(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_16(&e,) })
                == std::slice::from_raw_parts(
                    c"0".as_ptr(),
                    libcc2rs::c_str_len(c"0".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test3_37() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_17(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_18(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_19(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test3_38() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_20(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_21(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_22(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test3_39() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_23(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_24(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_25(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test3_40() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_26(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_27(&e,) })
                == std::slice::from_raw_parts(
                    c"5".as_ptr(),
                    libcc2rs::c_str_len(c"5".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_28(&e,) })
                == std::slice::from_raw_parts(
                    c"3".as_ptr(),
                    libcc2rs::c_str_len(c"3".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test4_41() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_17(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_18(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_19(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test4_42() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_20(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_21(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_22(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test4_43() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_23(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_24(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_25(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test4_44() {
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_26(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_27(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
    {
        let mut e: () = unimplemented!();
        assert!(
            (unsafe { to_string_28(&e,) })
                == std::slice::from_raw_parts(
                    c"1".as_ptr(),
                    libcc2rs::c_str_len(c"1".as_ptr()) + 1
                )
                .to_vec()
        );
    }
}
pub unsafe fn test_implicit_45() {
    const _: () = assert!(true, "test_convertible<E>()");;
    assert!(unimplemented!());
    const _: () = assert!(true, "!test_convertible<E, T>()");;
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
    (unsafe { test1_29() });
    (unsafe { test1_30() });
    (unsafe { test1_31() });
    (unsafe { test1_32() });
    (unsafe { test2_33() });
    (unsafe { test2_34() });
    (unsafe { test2_35() });
    (unsafe { test2_36() });
    (unsafe { test3_37() });
    (unsafe { test3_38() });
    (unsafe { test3_39() });
    (unsafe { test3_40() });
    (unsafe { test4_41() });
    (unsafe { test4_42() });
    (unsafe { test4_43() });
    (unsafe { test4_44() });
    (unsafe { test_implicit_45() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
