extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn foo_0() -> i32 {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut kX1_4)) += 1;
    return (((*std::cell::LazyCell::force_mut(&mut *&raw mut kX1_4))
        + (*std::cell::LazyCell::force_mut(&mut *&raw mut kX2_5)))
        + (*std::cell::LazyCell::force_mut(&mut *&raw mut static_i_1)));
}
static mut static_i_1: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
static mut static_f_2: std::cell::LazyCell<f32> = std::cell::LazyCell::new(|| unsafe { 0_f32 });
static mut static_b_3: std::cell::LazyCell<bool> = std::cell::LazyCell::new(|| unsafe { false });
static mut kX1_4: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 1 });
static mut kX2_5: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 2 });
pub unsafe fn from_local_class_6() -> i32 {
    return ((unsafe { S_8::get(&<S_8>::default()) }) + (unsafe { S_8::get(&<S_8>::default()) }));
}
static mut x_7: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 3 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct S_8 {}
impl S_8 {
    pub unsafe fn get(&self) -> i32 {
        return (*std::cell::LazyCell::force_mut(&mut *&raw mut x_7)).prefix_inc();
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (if ((((unsafe { foo_0() }) + (unsafe { foo_0() })) + (unsafe { foo_0() })) == (15)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"foo() + foo() + foo() == 15".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { from_local_class_6() }) == (9)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"from_local_class() == 9".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((unsafe { from_local_class_6() }) == (13)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_9(
                c"from_local_class() == 13".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    return 0;
}
pub unsafe fn __assert_fail_9(
    mut __assertion: *const libc::c_char,
    mut __file: *const libc::c_char,
    mut __line: u32,
    mut __function: *const libc::c_char,
) {
    unimplemented!()
}
pub unsafe fn __cpp2rust_init_globals() {}
