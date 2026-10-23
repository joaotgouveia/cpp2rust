extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, MoveAssignUnsafe)]
pub struct MoveOnly {
    data_: (),
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<()>) -> Self {
        let mut data: () = data.unwrap_or_else(|| unsafe { 1_() });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0_();
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0_();
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> () {
        return self.data_;
    }
}
impl Default for MoveOnly {
    fn default() -> Self {
        unsafe { MoveOnly::new(None) }
    }
}
pub unsafe fn test_0() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub unsafe fn test_1() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub unsafe fn test_2() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
}
pub unsafe fn test_3() {
    const _: () = assert!(true, "std::is_copy_constructible<Wrap>::value");;
    const _: () = assert!(true, "std::is_copy_assignable<Wrap>::value");;
    const _: () = assert!(true, "std::is_trivially_copyable<Wrap>::value");;
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
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
