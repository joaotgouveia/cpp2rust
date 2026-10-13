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
pub struct Vtable {
    pub create: Option<unsafe fn(i32) -> *mut ::libc::c_void>,
    pub get: Option<unsafe fn(*mut ::libc::c_void) -> i32>,
    pub destroy: Option<unsafe fn(*mut ::libc::c_void)>,
}
pub static mut storage_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 0_i32 });
pub unsafe fn int_create_1(mut val: i32) -> *mut ::libc::c_void {
    (*std::cell::LazyCell::force_mut(&mut *&raw mut storage_0)) = val;
    return ((&raw mut (*std::cell::LazyCell::force_mut(&mut *&raw mut storage_0)) as *mut i32)
        as *mut ::libc::c_void);
}
pub unsafe fn int_get_2(mut p: *mut ::libc::c_void) -> i32 {
    return (*(p as *mut i32));
}
pub unsafe fn int_destroy_3(mut p: *mut ::libc::c_void) {
    (*(p as *mut i32)) = 0;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut vt: Vtable = Vtable {
        create: Some(int_create_1),
        get: Some(int_get_2),
        destroy: Some(int_destroy_3),
    };
    (if !((vt.create).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"vt.create != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((vt.get).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"vt.get != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if !((vt.destroy).is_none()) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"vt.destroy != nullptr".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut obj: *mut ::libc::c_void = (unsafe { (vt.create).unwrap()(42) });
    (if ((unsafe { (vt.get).unwrap()(obj) }) == (42)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"vt.get(obj) == 42".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (unsafe { (vt.destroy).unwrap()(obj) });
    (if ((*std::cell::LazyCell::force_mut(&mut *&raw mut storage_0)) == (0)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"storage == 0".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (vt.get) = None;
    (if (vt.get).is_none() {
        &(0);
    } else {
        (unsafe {
            __assert_fail_4(
                c"vt.get == nullptr".as_ptr(),
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
    std::cell::LazyCell::force(&*&raw const storage_0);
}
