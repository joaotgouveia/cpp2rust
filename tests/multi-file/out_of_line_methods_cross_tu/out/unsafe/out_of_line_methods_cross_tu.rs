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
pub struct S {
    pub v: i32,
}
impl S {
    pub unsafe fn get(&self) -> i32 {
        return self.v;
    }
}
pub unsafe trait Base {
    unsafe fn apply(&mut self, x: i32) -> i32;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Derived {
    pub factor: i32,
}
impl Derived {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}
pub unsafe fn pair_diff_0(mut p: *const Pair) -> i32 {
    return (((*p).first) - ((*p).second));
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut s: S = S::new({ 1 });
    (if ((unsafe { S::get(&s) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.get() == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { S::set(&mut s, 4) });
    (if ((unsafe { S::get(&s) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.get() == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::add(&mut s, 2) }) == (6)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"s.add(2) == 6".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut derived: Derived = Derived::new({ 3 });
    let mut base: *mut dyn Base = (&mut derived as *mut Derived);
    (if ((unsafe { (*base).apply(5) }) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"base->apply(5) == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pair: Pair = Pair {
        first: 7,
        second: 3,
    };
    (if ((unsafe { pair_sum_2((&mut pair as *mut Pair).cast_const()) }) == (10)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"pair_sum(&pair) == 10".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { pair_diff_0((&mut pair as *mut Pair).cast_const()) }) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_1(
                c"pair_diff(&pair) == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
impl S {
    pub unsafe fn new(mut x: i32) -> Self {
        let mut this = Self { v: x };
        this
    }
}
impl Derived {
    pub unsafe fn new(mut factor: i32) -> Self {
        let mut this = Self { factor: factor };
        this
    }
}
impl S {}
impl S {
    pub unsafe fn destructor(&mut self) {}
}
impl S {
    pub unsafe fn set(&mut self, mut x: i32) {
        self.v = x;
    }
}
impl S {
    pub unsafe fn add(&mut self, mut x: i32) -> i32 {
        self.v += x;
        return self.v;
    }
}
impl Derived {}
pub unsafe fn pair_sum_2(mut p: *const Pair) -> i32 {
    return (((*p).first) + ((*p).second));
}
pub unsafe fn __assert_fail_1(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
unsafe impl Base for Derived {
    unsafe fn apply(&mut self, x: i32) -> i32 {
        return ((self.factor) * (x));
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
