extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg, MoveCtorUnsafe, CppFromUnsafe, MoveAssignUnsafe)]
#[cpp_from(i32, new, some_value)]
pub struct MoveOnly {
    data_: i32,
}
impl MoveOnly {
    pub unsafe fn new(mut data: Option<i32>) -> Self {
        let mut data: i32 = data.unwrap_or_else(|| unsafe { 1 });
        let mut this = Self { data_: data };
        this
    }
    pub unsafe fn move_from(x: *mut MoveOnly) -> Self {
        let mut this = Self { data_: (*x).data_ };
        (*x).data_ = 0;
        this
    }
    pub unsafe fn move_assign(&mut self, x: *mut MoveOnly) -> *mut MoveOnly {
        self.data_ = (*x).data_;
        (*x).data_ = 0;
        return &mut (*(self as *mut MoveOnly));
    }
    pub unsafe fn get(&self) -> i32 {
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_0() });
    (unsafe { test_1() });
    (unsafe { test_2() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
