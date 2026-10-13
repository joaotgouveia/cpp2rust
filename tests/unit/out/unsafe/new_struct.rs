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
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Triple {
    pub a: i32,
    pub b: i32,
    pub p: Pair,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut p: *mut Pair = (Box::leak(Box::new(Pair { x: 1, y: 2 })) as *mut Pair);
    let mut out: i32 = (((*p).x) + ((*p).y));
    {
        let __p = p;
        if !__p.is_null() {
            ::std::mem::drop(Box::from_raw(__p))
        }
    };
    (if ((out) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"out == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut t: Triple = Triple {
        a: 1,
        b: 0_i32,
        p: <Pair>::default(),
    };
    (if ((t.a) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t.a == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((t.b) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t.b == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((t.p.x) == (0)) && ((t.p.y) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"t.p.x == 0 && t.p.y == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q: *mut Triple = (Box::leak(Box::new(Triple {
        a: 2,
        b: 3,
        p: <Pair>::default(),
    })) as *mut Triple);
    (if (((*q).a) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"q->a == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*q).b) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"q->b == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if (((*q).p.x) == (0)) && (((*q).p.y) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"q->p.x == 0 && q->p.y == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    {
        let __p = q;
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
