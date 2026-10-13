extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
static mut C_inner_const_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct C {}
impl C {
    pub unsafe fn get(&mut self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut C_inner_const_0));
    }
}
pub static mut S_inner_const_1: std::cell::LazyCell<i32> =
    std::cell::LazyCell::new(|| unsafe { 2 });
pub type anon_3 = u32;
pub const anon_3_kValue: anon_3 = 3;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S {}
impl S {
    pub unsafe fn f() -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2));
    }
}
pub static mut S_counter_2: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 10 });
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut c: C = <C>::default();
    (if ((unsafe { C::get(&mut c) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"c.get() == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_inner_const_1)) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"S::inner_const == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut s: S = <S>::default();
    let mut p: *mut S = (&mut s as *mut S);
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_inner_const_1)) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"s.inner_const == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_inner_const_1)) == (2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"p->inner_const == 2".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_3_kValue as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"s.kValue == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((anon_3_kValue as i32) == (3)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"p->kValue == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2)) = 20;
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2)) == (20)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"S::counter == 20".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2)) += 5;
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2)) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"s.counter == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut S_counter_2)) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"S::counter == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::f() }) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"s.f() == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { S::f() }) == (25)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"p->f() == 25".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_4(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const C_inner_const_0);
    std::cell::LazyCell::force(&*&raw const S_inner_const_1);
    std::cell::LazyCell::force(&*&raw const S_counter_2);
}
