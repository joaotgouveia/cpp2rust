extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub type anon_0 = u32;
pub const anon_0_value: anon_0 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char_ref__const_char_ref_ {}
pub type anon_1 = u32;
pub const anon_1_value: anon_1 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_wchar_t_ref__const_wchar_t_ref_ {}
pub type anon_2 = u32;
pub const anon_2_value: anon_2 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char16_t_ref__const_char16_t_ref_ {}
pub type anon_3 = u32;
pub const anon_3_value: anon_3 = 1;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_macros_detail_is_same_const_char32_t_ref__const_char32_t_ref_ {}
pub unsafe fn test_4(mut s: *const libc::c_char, mut len: usize) -> bool {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(((unimplemented!() as usize) == (len)));
    assert!((((*unimplemented!()) as i32) == ((*s.offset((0) as isize)) as i32)));
    return ((unimplemented!()) == (s));
}
pub unsafe fn test_5(mut s: *const i32, mut len: usize) -> bool {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(((unimplemented!() as usize) == (len)));
    assert!((((*unimplemented!()) as i32) == (*s.offset((0) as isize))));
    return ((unimplemented!()) == (s));
}
pub unsafe fn test_6(mut s: *const u16, mut len: usize) -> bool {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(((unimplemented!() as usize) == (len)));
    assert!((((*unimplemented!()) as i32) == ((*s.offset((0) as isize)) as i32)));
    return ((unimplemented!()) == (s));
}
pub unsafe fn test_7(mut s: *const u32, mut len: usize) -> bool {
    let mut sv: () = unimplemented!();
    const _: () = assert!(true, "");;
    const _: () = assert!(true, "");;
    assert!(((unimplemented!() as usize) == (len)));
    assert!((((*unimplemented!()) as u32) == (*s.offset((0) as isize))));
    return ((unimplemented!()) == (s));
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
    assert!((unsafe { test_4(c"ABCDE".as_ptr(), 5_usize,) }));
    assert!((unsafe { test_4(c"a".as_ptr(), 1_usize,) }));
    assert!(
        (unsafe {
            test_5(
                [
                    65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
                ]
                .as_ptr(),
                5_usize,
            )
        })
    );
    assert!((unsafe { test_5([97 as i32, 0 as i32,].as_ptr(), 1_usize,) }));
    assert!(
        (unsafe {
            test_6(
                [
                    65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
                ]
                .as_ptr(),
                5_usize,
            )
        })
    );
    assert!((unsafe { test_6([97 as u16, 0 as u16,].as_ptr(), 1_usize,) }));
    assert!(
        (unsafe {
            test_7(
                [
                    65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
                ]
                .as_ptr(),
                5_usize,
            )
        })
    );
    assert!((unsafe { test_7([97 as u32, 0 as u32,].as_ptr(), 1_usize,) }));
    {
        let sv: () = unimplemented!();
        const _: () = assert!(true, "sv.length() == 2");;
        const _: () = assert!(true, "sv.front() == \'A\'");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
