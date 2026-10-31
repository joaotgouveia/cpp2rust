extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct foo {
    val_: i32,
}
impl foo {
    pub unsafe fn new() -> Self {
        let mut this = Self { val_: 3 };
        this
    }
    pub unsafe fn get(&self) -> i32 {
        return self.val_;
    }
}
impl Default for foo {
    fn default() -> Self {
        unsafe { foo::new() }
    }
}
pub unsafe fn test_0() -> bool {
    {
        let mut p1: Option<Box<[i32]>> = Some((0..5_usize).map(|_| 0_i32).collect::<Box<[_]>>());
        let mut i: i32 = 0;
        'loop_: while ((i) < (5)) {
            assert!(
                ((*p1
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut i32)
                    .add((i as usize)))
                    == (0))
            );
            i.prefix_inc();
        }
    }
    {
        let mut p2: Option<Box<[Vec<libc::c_char>]>> = Some(
            (0..5_usize)
                .map(|_| vec![libc::c_char::default()])
                .collect::<Box<[_]>>(),
        );
        let mut i: i32 = 0;
        'loop_: while ((i) < (5)) {
            assert!(
                (((*p2
                    .as_deref()
                    .map_or(::std::ptr::null_mut(), |s| s.as_ptr()
                        as *mut Vec<libc::c_char>)
                    .add((i as usize)))
                .len()
                    - 1)
                    == (0_usize))
            );
            i.prefix_inc();
        }
    }
    {
        let mut p3: Option<Box<[foo]>> =
            Some((0..7_usize).map(|_| <foo>::default()).collect::<Box<[_]>>());
        let mut i: i32 = 0;
        'loop_: while ((i) < (7)) {
            assert!(
                ((unsafe {
                    foo::get(
                        &(*p3
                            .as_deref()
                            .map_or(::std::ptr::null_mut(), |s| s.as_ptr() as *mut foo)
                            .add((i as usize))),
                    )
                }) == (3))
            );
            i.prefix_inc();
        }
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
