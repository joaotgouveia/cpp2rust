extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_0(
    mut bad: i32,
    mut args_1: i32,
    mut args_2: i32,
    mut args_3: i32,
    mut args_4: i32,
) {
    let mut set: () = unimplemented!();
    let mut vals: [i32; 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut i32 = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut i32 = &mut (*v);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
}
pub unsafe fn test_1(
    mut bad: libc::c_char,
    mut args_1: libc::c_char,
    mut args_2: libc::c_char,
    mut args_3: libc::c_char,
    mut args_4: libc::c_char,
) {
    let mut set: () = unimplemented!();
    let mut vals: [libc::c_char; 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut libc::c_char = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut libc::c_char = &mut (*v);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
}
pub unsafe fn test_2(
    mut bad: i32,
    mut args_1: i32,
    mut args_2: i32,
    mut args_3: i32,
    mut args_4: i32,
) {
    let mut set: () = unimplemented!();
    let mut vals: [i32; 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut i32 = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut i32 = &mut (*v);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
}
pub unsafe fn test_3(
    mut bad: libc::c_char,
    mut args_1: libc::c_char,
    mut args_2: libc::c_char,
    mut args_3: libc::c_char,
    mut args_4: libc::c_char,
) {
    let mut set: () = unimplemented!();
    let mut vals: [libc::c_char; 4] = [args_1, args_2, args_3, args_4];
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut libc::c_char = &mut (*v);
        unimplemented!();
    }
    'loop_: for v in IterRange::new(
        vals.as_mut_ptr(),
        vals.as_mut_ptr().offset((4_i64) as isize),
    ) {
        let v: *mut libc::c_char = &mut (*v);
        assert!(unimplemented!());
    }
    assert!(!(unimplemented!()));
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct E {
    pub a: i32,
    pub b: f64,
    pub c: libc::c_char,
}
impl Default for E {
    fn default() -> Self {
        unsafe {
            E {
                a: 1,
                b: 1_f64,
                c: (1 as libc::c_char),
            }
        }
    }
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
        (unsafe { test_0(14, 10, 11, 12, 13) });
        (unsafe {
            test_1(
                ('e' as libc::c_char),
                ('a' as libc::c_char),
                ('b' as libc::c_char),
                ('c' as libc::c_char),
                ('d' as libc::c_char),
            )
        });
    }
    {
        (unsafe { test_2(14, 10, 11, 12, 13) });
        (unsafe {
            test_3(
                ('e' as libc::c_char),
                ('a' as libc::c_char),
                ('b' as libc::c_char),
                ('c' as libc::c_char),
                ('d' as libc::c_char),
            )
        });
    }
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
