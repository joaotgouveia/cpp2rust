extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut gi_0: std::cell::LazyCell<i32> = std::cell::LazyCell::new(|| unsafe { 5 });
pub static mut gf_1: std::cell::LazyCell<f32> = std::cell::LazyCell::new(|| unsafe { 8_f32 });

pub fn main() {
    let mut args: Vec<Vec<u8>> = std::env::args()
        .map(|arg| arg.as_bytes().to_vec())
        .collect();
    args.iter_mut().for_each(|v| v.push(0));
    let mut argv: Vec<*mut libc::c_char> = args
        .iter()
        .map(|arg| arg.as_ptr() as *mut libc::c_char)
        .collect();
    argv.push(::std::ptr::null_mut());
    unsafe {
        __cpp2rust_init_globals();
        ::std::process::exit(main_0((argv.len() - 1) as i32, argv.as_mut_ptr()) as i32)
    }
}
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    const _: () = assert!(true, "std::launder(&gi) == &gi");;
    const _: () = assert!(true, "std::launder(&gf) == &gf");;
    let mut i: *const i32 =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut gi_0)) as *const i32);
    let mut f: *const f32 =
        (&raw const (*std::cell::LazyCell::force_mut(&mut *&raw mut gf_1)) as *const f32);
    const _: () = assert!(
        true,
        "std::is_same<decltype(i), decltype(std::launder(i))>::value"
    );;
    const _: () = assert!(
        true,
        "std::is_same<decltype(f), decltype(std::launder(f))>::value"
    );;
    assert!(((i) == (i)));
    assert!(((f) == (f)));
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const gi_0);
    std::cell::LazyCell::force(&*&raw const gf_1);
}
