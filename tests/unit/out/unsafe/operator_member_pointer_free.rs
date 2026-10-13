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
pub struct Inner {
    pub x: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {
    pub data: [i32; 3],
    pub inner: Inner,
}
pub unsafe fn operator_deref_0(s: *mut S) -> *mut Inner {
    return &mut (*s).inner;
}
pub unsafe fn operator_addr_1(s: *mut S) -> *mut i32 {
    return (&mut (*s).data[(0) as usize] as *mut i32);
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S {
        data: [1, 2, 3],
        inner: Inner { x: 9 },
    };
    (if (((*(unsafe {
        let _s: *mut S = &mut s;
        operator_deref_0(_s)
    }))
    .x) == (9))
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"(*s).x == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*(unsafe {
        let _s: *mut S = &mut s;
        operator_deref_0(_s)
    }))
    .x = 10;
    (if ((s.inner.x) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.inner.x == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: *mut i32 = (unsafe {
        let _s: *mut S = &mut s;
        operator_addr_1(_s)
    });
    (if ((*p) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"*p == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*p) = 5;
    (if ((s.data[(0) as usize]) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"s.data[0] == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_2(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
