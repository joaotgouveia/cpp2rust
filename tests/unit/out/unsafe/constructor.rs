extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut total_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0 });
#[repr(C)]
#[derive(Clone, VaArg, DestructorUnsafe, FnPtrArg, Default)]
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn const_method(&self) -> i32 {
        return ((self.v) * (2));
    }
    pub unsafe fn mut_method(&mut self) {
        self.v += 1;
    }
    pub unsafe fn new(mut init: i32) -> Self {
        let mut this = Self { v: init };
        (unsafe { S::mut_method(&mut this) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(&mut this) });
        this
    }
    pub unsafe fn destructor(&mut self) {
        (unsafe { S::mut_method(self) });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) +=
            (unsafe { S::const_method(self) });
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    pub unsafe fn new_1(mut x: i32, mut y: i32) -> Self {
        let mut this = Self { x: x, y: y };
        this
    }
    pub unsafe fn new_2(mut v: i32) -> Self {
        let mut this = Point::new_1({ v }, { ((v) + (1)) });
        this.y *= 10;
        this
    }
    pub unsafe fn new_3() -> Self {
        let mut this = Point::new_2({ 4 });
        this.x += 100;
        this
    }
}
impl Default for Point {
    fn default() -> Self {
        unsafe { Point::new_3() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    {
        let mut s: S = S::new({ 3 });
        let _dtor_s = ScopedDestructorUnsafe::new(&raw mut s, S::destructor);
        (if ((s.v) == (4)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_1(
                    c"s.v == 4".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
        (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (8)) {
            &(0);
        } else {
            (unsafe {
                __assert_fail_1(
                    c"total == 8".as_ptr(),
                    concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                    line!() as u32,
                    c"int main()".as_ptr(),
                )
            });
        });
    }
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut total_0)) == (18)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"total == 18".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut p: Point = Point::new_3();
    (if ((p.x) == (104)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p.x == 104".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((p.y) == (50)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"p.y == 50".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut q: Point = Point::new_2({ 7 });
    (if ((q.x) == (7)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"q.x == 7".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((q.y) == (80)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"q.y == 80".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const total_0);
}
