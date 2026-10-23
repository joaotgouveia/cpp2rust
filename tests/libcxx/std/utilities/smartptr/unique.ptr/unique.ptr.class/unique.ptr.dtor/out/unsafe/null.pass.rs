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
    state_: (),
}
impl Deleter {
    pub unsafe fn new() -> Self {
        let mut this = Self { state_: 0_() };
        this
    }
    pub unsafe fn state(&self) -> () {
        return self.state_;
    }
    pub unsafe fn operator_call(&mut self, mut _a0: ()) {
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
    unimplemented!();
    {
        let mut p: () = std_unique_ptr_int__Deleter_ref_::new({ unimplemented!() }, { &mut d });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_basic_1() {
    let mut d: Deleter = Deleter::new();
    unimplemented!();
    {
        let mut p: () =
            std_unique_ptr_intarrarr__Deleter_ref_::new({ Default::default() }, { &mut d });
        unimplemented!();
        unimplemented!();
    }
    unimplemented!();
}
pub unsafe fn test_2() -> () {
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
unsafe fn main_0(mut _a0: (), mut _a1: *mut *mut ()) -> () {
    (unsafe { test_2() });
    const _: () = assert!(true, "test()");;
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
