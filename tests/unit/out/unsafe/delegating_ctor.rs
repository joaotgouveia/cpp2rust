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
pub struct Foo {
    pub a_: i32,
    pub b_: i32,
}
impl Foo {
    pub unsafe fn new_1(mut a: i32, mut b: i32) -> Self {
        let mut this = Self { a_: a, b_: b };
        this
    }
    pub unsafe fn new_2(mut a: i32, mut count: *mut i32) -> Self {
        let mut this = Foo::new_1({ a }, { 0 });
        (*count) += 1;
        this
    }
    pub unsafe fn new_3() -> Self {
        let mut this = Foo::new_1({ 7 }, { 8 });
        this.a_ += 1;
        this
    }
}
impl Default for Foo {
    fn default() -> Self {
        unsafe { Foo::new_3() }
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut count: i32 = 0;
    let mut f: Foo = Foo::new_2({ 3 }, { (&mut count as *mut i32) });
    (if ((f.a_) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"f.a_ == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((f.b_) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"f.b_ == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((count) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"count == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut g: Foo = Foo::new_3();
    (if ((g.a_) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"g.a_ == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((g.b_) == (8)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"g.b_ == 8".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
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
