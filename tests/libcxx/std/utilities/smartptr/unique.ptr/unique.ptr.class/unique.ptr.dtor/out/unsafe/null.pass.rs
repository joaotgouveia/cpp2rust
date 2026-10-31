extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(FnPtrArg)]
pub struct Deleter {
    state_: i32,
}
impl Deleter {
    pub unsafe fn new() -> Self {
        let mut this = Self { state_: 0 };
        this
    }
    pub unsafe fn state(&self) -> i32 {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut _a0: *mut ::libc::c_void) {
        self.state_.prefix_inc();
    }
}
impl Default for Deleter {
    fn default() -> Self {
        unsafe { Deleter::new() }
    }
}
pub unsafe fn test_basic_0() {
    let mut d: Deleter = Deleter::new();
    assert!(((unsafe { Deleter::state(&d,) }) == (0)));
    {
        let mut p: Option<Box<i32>> = unimplemented!();
        assert!(
            (p.as_deref_mut()
                .map_or(::std::ptr::null_mut(), |v| v as *mut i32))
            .is_null()
        );
        assert!(((unimplemented!()) == (&mut d as *mut Deleter)));
    }
    assert!(((unsafe { Deleter::state(&d,) }) == (0)));
}
pub unsafe fn test_basic_1() {
    let mut d: Deleter = Deleter::new();
    assert!(((unsafe { Deleter::state(&d,) }) == (0)));
    {
        let mut p: Option<Box<[i32]>> = unimplemented!();
        assert!(
            (p.as_deref_mut()
                .map_or(::std::ptr::null_mut(), |s| s.as_mut_ptr()))
            .is_null()
        );
        assert!(((unimplemented!()) == (&mut d as *mut Deleter)));
    }
    assert!(((unsafe { Deleter::state(&d,) }) == (0)));
}
pub unsafe fn test_2() -> bool {
    (unsafe { test_basic_0() });
    (unsafe { test_basic_1() });
    return true;
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
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
