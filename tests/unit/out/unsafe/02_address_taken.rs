extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[derive(VaArg, FnPtrArg)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: i64,
    pub __clang_max_align_nonce2: f64,
}
impl Clone for max_align_t {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
impl Default for max_align_t {
    fn default() -> Self {
        unimplemented!()
    }
}
pub type std_byte = u8;
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut b: i32 = 2;
    let mut b_ptr: *mut i32 = (&mut b as *mut i32);
    (*b_ptr) = 3;
    let mut b_ptr_ptr: *mut *mut i32 = (&mut b_ptr as *mut *mut i32);
    (*(*b_ptr_ptr)) = 4;
    (*b_ptr) = (*(*b_ptr_ptr));
    let mut offset: usize =
        ((((b_ptr as usize - b_ptr as usize) / ::std::mem::size_of::<i32>()) as i64) as usize);
    (if ((b) == (4)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_0(
                c"b == 4".as_ptr(),
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
