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
pub unsafe fn out_param_0(mut out: Option<*mut usize>) {
    let mut out: *mut usize = out.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(out).is_null() {
        (*out) = 4_usize;
    }
}
pub unsafe fn parse_1(mut v: i32, mut idx: Option<*mut usize>) -> i32 {
    let mut idx: *mut usize = idx.unwrap_or_else(|| unsafe { std::ptr::null_mut() });
    if !(idx).is_null() {
        (*idx) = 3_usize;
    }
    return v;
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    let mut v2: usize = 0_usize;
    (unsafe { out_param_0(Some((&mut v2 as *mut usize))) });
    (if ((v2) == (4_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"v2 == 4".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut pidx: usize = 0_usize;
    (if ((unsafe { parse_1(1, Some((&mut pidx as *mut usize))) }) == (1)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"ns::parse(1, &pidx) == 1".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    (if ((pidx) == (3_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"pidx == 3".as_ptr(),
                concat!(file!(), "\0").as_ptr() as *const libc::c_char,
                line!() as u32,
                c"int main()".as_ptr(),
            )
        });
    });
    let mut sv: usize = 7_usize;
    let mut sp: *mut usize = (&mut sv as *mut usize);
    let mut spp: *mut *mut usize = (&mut sp as *mut *mut usize);
    (if ((*(*spp)) == (7_usize)) {
        &(0);
    } else {
        (unsafe {
            __assert_fail_2(
                c"**spp == 7".as_ptr(),
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
