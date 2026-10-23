extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe, Default)]
pub struct X {}
impl X {
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) = true;
    }
}
pub static mut dtor_called_0: std::cell::LazyCell<()> =
    std::cell::LazyCell::new(|| unsafe { false });
pub unsafe fn check_reset_1() -> () {
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "noexcept(opt.reset()) == true");;
        (unsafe { (opt as std___optional_destruct_base_int__true_).reset() });
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        (unsafe { (opt as std___optional_destruct_base_int__true_).reset() });
        unimplemented!();
    }
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
    (unsafe { check_reset_1() });
    const _: () = assert!(true, "check_reset()");;
    {
        let mut opt: () = unimplemented!();
        const _: () = assert!(true, "noexcept(opt.reset()) == true");;
        unimplemented!();
        (unsafe { (opt as std___optional_destruct_base_X__false_).reset() });
        unimplemented!();
        unimplemented!();
    }
    {
        let mut opt: () = unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) = false;
        (unsafe { (opt as std___optional_destruct_base_X__false_).reset() });
        unimplemented!();
        unimplemented!();
        (*std::cell::LazyCell::force_mut(&mut *&raw mut dtor_called_0)) = false;
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const dtor_called_0);
}
