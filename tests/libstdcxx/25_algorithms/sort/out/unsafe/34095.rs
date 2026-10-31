extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test01_0() {
    let mut v: Vec<Vec<i32>> = (0..(20000_usize) as usize)
        .map(|_| Default::default())
        .collect::<Vec<_>>();
    {
        let __first = v.begin();
        let __n = v.end().iter_distance(&__first);
        let mut __order: Vec<isize> = (0..__n).collect();
        __order.sort_by(|&__x, &__y| (*__first.iter_at(__x)).cmp(&*__first.iter_at(__y)));
        let __values: Vec<_> = __order
            .iter()
            .map(|&__i| std::ptr::read(__first.iter_at(__i)))
            .collect();
        for (__i, __v) in (0..__n).zip(__values) {
            std::ptr::write(__first.iter_at(__i), __v);
        }
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test01_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
