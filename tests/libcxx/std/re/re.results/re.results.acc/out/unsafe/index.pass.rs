extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(mut syntax: ()) {
    let mut m: () = unimplemented!();
    let s: [libc::c_char; 12] = std::mem::transmute(*b"abcdefghijk\0");
    assert!(unimplemented!());
    assert!(((unimplemented!()) == (4_())));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (s.as_ptr().offset((2) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (s.as_ptr().offset((9) as isize)))
    );
    assert!((((*unimplemented!()).matched as i32) == (true as i32)));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (s.as_ptr().offset((4) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (s.as_ptr().offset((7) as isize)))
    );
    assert!((((*unimplemented!()).matched as i32) == (true as i32)));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (s.as_ptr().offset((4) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (s.as_ptr().offset((5) as isize)))
    );
    assert!((((*unimplemented!()).matched as i32) == (true as i32)));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (s.as_ptr().offset((11) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (s.as_ptr().offset((11) as isize)))
    );
    assert!((((*unimplemented!()).matched as i32) == (false as i32)));
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).0)
            == (s.as_ptr().offset((11) as isize)))
    );
    assert!(
        ((((*unimplemented!()) as (*const libc::c_char, *const libc::c_char)).1)
            == (s.as_ptr().offset((11) as isize)))
    );
    assert!((((*unimplemented!()).matched as i32) == (false as i32)));
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
    (unsafe { test_0(unimplemented!()) });
    (unsafe { test_0(unimplemented!()) });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
