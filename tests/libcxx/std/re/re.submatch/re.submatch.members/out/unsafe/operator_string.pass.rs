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
    {
        let mut sm: () = unimplemented!();
        let mut str: () = unimplemented!();
        assert!(Vec::len(&str) <= 1);
        let s: [libc::c_char; 4] = [
            ('1' as libc::c_char),
            ('2' as libc::c_char),
            ('3' as libc::c_char),
            (0 as libc::c_char),
        ];
        (sm as (*const libc::c_char, *const libc::c_char)).0 = s.as_ptr();
        (sm as (*const libc::c_char, *const libc::c_char)).1 = s.as_ptr().offset((3) as isize);
        sm.matched = true;
        str = unimplemented!();
        assert!(unimplemented!());
    }
    {
        let mut sm: () = unimplemented!();
        let mut str: () = unimplemented!();
        assert!(Vec::len(&str) <= 1);
        let s: [i32; 4] = [
            (('1' as libc::c_char) as i32),
            (('2' as libc::c_char) as i32),
            (('3' as libc::c_char) as i32),
            0_i32,
        ];
        (sm as (*const i32, *const i32)).0 = s.as_ptr();
        (sm as (*const i32, *const i32)).1 = s.as_ptr().offset((3) as isize);
        sm.matched = true;
        str = unimplemented!();
        assert!(unimplemented!());
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
