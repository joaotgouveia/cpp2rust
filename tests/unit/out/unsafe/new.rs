extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub x: i32,
    pub y: i32,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut x: *mut i32 = (Box::leak(Box::new(5)) as *mut i32);
    let mut out: i32 = (*x);
    {
        let __p = x;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    (if ((out) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"out == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut y: *mut i32 = (Box::leak(Box::new(0_i32)) as *mut i32);
    (*y) = 9;
    (if ((*y) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"*y == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __p = y;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut p: *mut Pair = (Box::leak(Box::new(<Pair>::default())) as *mut Pair);
    (*p).x = 1;
    (*p).y = 2;
    (if ((((*p).x) + ((*p).y)) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"p->x + p->y == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    let mut nullpointer: *mut i32 = std::ptr::null_mut();
    {
        let __p = nullpointer;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    return 0;
}
pub unsafe fn __assert_fail_0(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
