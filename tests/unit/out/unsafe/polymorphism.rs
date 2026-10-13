extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe trait Animal {
    unsafe fn bark(&self) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Dog {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Cat {}
impl Cat {
    unsafe fn meow(&self) -> bool {
        return true;
    }
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut dog: Dog = <Dog>::default();
    let mut animal: *mut dyn Animal = (&mut dog as *mut Dog);
    let mut eat1: bool = (unsafe { (*(animal).cast_const()).bark() });
    let mut cat: Cat = <Cat>::default();
    animal = (&mut cat as *mut Cat);
    let mut eat2: bool = (unsafe { (*(animal).cast_const()).bark() });
    (if (eat1) && (!(eat2)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"eat1 && !eat2".as_ptr(),
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
unsafe impl Animal for Cat {
    unsafe fn bark(&self) -> bool {
        return false;
    }
}
unsafe impl Animal for Dog {
    unsafe fn bark(&self) -> bool {
        return true;
    }
}
pub unsafe fn __cpp2rust_init_globals() {}
