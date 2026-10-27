extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test_pointer_0() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub unsafe fn test_pointer_1() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub unsafe fn test_pointer_2() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub unsafe fn test_void_pointer_3() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub unsafe fn test_void_pointer_4() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
pub unsafe fn test_void_pointer_5() {
    let mut vp: () = unimplemented!();
    let mut cvp: () = unimplemented!();
    (&(vp));
    (&(cvp));
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(vp <= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= vp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp == cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp != cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp > cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp >= cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp < cvp)>::value");;
    const _: () = assert!(true, "std::is_same<bool, decltype(cvp <= cvp)>::value");;
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct Foo {
    pub x: (),
}
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_pointer_0() });
    (unsafe { test_pointer_1() });
    (unsafe { test_pointer_2() });
    (unsafe { test_void_pointer_3() });
    (unsafe { test_void_pointer_4() });
    (unsafe { test_void_pointer_5() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
