extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut s: *const libc::c_char, mut len: usize) {
    let mut sv: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (len)));
    assert!(((unimplemented!()) == (s)));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_1(mut s: *const i32, mut len: usize) {
    let mut sv: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (len)));
    assert!(((unimplemented!()) == (s)));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_2(mut s: *const u16, mut len: usize) {
    let mut sv: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (len)));
    assert!(((unimplemented!()) == (s)));
    assert!(((unimplemented!()) == (unimplemented!())));
}
pub unsafe fn test_3(mut s: *const u32, mut len: usize) {
    let mut sv: () = unimplemented!();
    assert!(((unimplemented!() as usize) == (len)));
    assert!(((unimplemented!()) == (s)));
    assert!(((unimplemented!()) == (unimplemented!())));
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
    (unsafe { test_0(c"ABCDE".as_ptr(), 5_usize) });
    (unsafe { test_0(c"a".as_ptr(), 1_usize) });
    (unsafe {
        test_1(
            [
                65 as i32, 66 as i32, 67 as i32, 68 as i32, 69 as i32, 0 as i32,
            ]
            .as_ptr(),
            5_usize,
        )
    });
    (unsafe { test_1([97 as i32, 0 as i32].as_ptr(), 1_usize) });
    (unsafe {
        test_2(
            [
                65 as u16, 66 as u16, 67 as u16, 68 as u16, 69 as u16, 0 as u16,
            ]
            .as_ptr(),
            5_usize,
        )
    });
    (unsafe { test_2([97 as u16, 0 as u16].as_ptr(), 1_usize) });
    (unsafe {
        test_3(
            [
                65 as u32, 66 as u32, 67 as u32, 68 as u32, 69 as u32, 0 as u32,
            ]
            .as_ptr(),
            5_usize,
        )
    });
    (unsafe { test_3([97 as u32, 0 as u32].as_ptr(), 1_usize) });
    {
        let s: *const libc::c_char = c"ABC".as_ptr();
        let sv: () = unimplemented!();
        const _: () = assert!(true, "sv.length() == 2");;
        const _: () = assert!(true, "sv.data() == s");;
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
