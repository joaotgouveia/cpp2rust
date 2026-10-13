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
pub struct Outer_Named {
    pub a: i32,
    pub b: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_0 {
    pub c: i32,
    pub d: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_1 {
    pub g: i32,
    pub h: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_2 {
    pub e: i32,
    pub f: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_4 {
    pub j: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_5 {
    pub k: i32,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_3 {
    pub i: i32,
    pub inner_named: anon_4,
    pub anon_5: anon_5,
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Outer {
    pub named: Outer_Named,
    pub anonymous_named_0: anon_0,
    pub anonymous_named_1: anon_1,
    pub anon_2: anon_2,
    pub anon_3: anon_3,
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut o: Outer = <Outer>::default();
    o.named.a = 1;
    o.named.b = 2;
    o.anonymous_named_0.c = 3;
    o.anonymous_named_0.d = 4;
    o.anonymous_named_1.g = 5;
    o.anonymous_named_1.h = 6;
    o.anon_2.e = 7;
    o.anon_2.f = 8;
    o.anon_3.i = 9;
    o.anon_3.inner_named.j = 10;
    o.anon_3.anon_5.k = 11;
    (if ((o.named.a) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.named.a == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.named.b) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.named.b == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anonymous_named_0.c) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.anonymous_named_0.c == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anonymous_named_0.d) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.anonymous_named_0.d == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anonymous_named_1.g) == (5)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.anonymous_named_1.g == 5".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anonymous_named_1.h) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.anonymous_named_1.h == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anon_2.e) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.e == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anon_2.f) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.f == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anon_3.i) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.i == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anon_3.inner_named.j) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.inner_named.j == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((o.anon_3.anon_5.k) == (11)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"o.k == 11".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s: anon_7 = <anon_7>::default();
    s.x = 1;
    s.z = 2;
    (if ({
        s.x = 1;
        s.x
    } != 0)
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"s.x = 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ({
        s.z = 2;
        s.z
    } != 0)
    {
        &(0);
    } else {
        (unsafe {
            __assert_fail_6(
                c"s.z = 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct anon_7 {
    pub x: i32,
    pub z: i32,
}
pub unsafe fn __assert_fail_6(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
