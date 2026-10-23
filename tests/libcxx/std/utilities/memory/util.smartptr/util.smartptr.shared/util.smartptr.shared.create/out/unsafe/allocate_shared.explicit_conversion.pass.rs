extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitAllocator_int_ {}
impl ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___ {
    pub unsafe fn new_1(mut _a0: ExplicitAllocator_int_) -> Self {
        let mut this = Self {};
        this
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___ {}
impl ExplicitAllocator_std___shared_ptr_emplace_int__ExplicitAllocator_int___ {
    pub unsafe fn allocate(
        &mut self,
        mut n: (),
    ) -> *mut std___shared_ptr_emplace_int__ExplicitAllocator_int__ {
        return unimplemented!();
    }
    pub unsafe fn deallocate(
        &mut self,
        mut ptr: *mut std___shared_ptr_emplace_int__ExplicitAllocator_int__,
        mut n: (),
    ) {
        unimplemented!();
        return;
    }
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
    let mut ptr: () = unimplemented!();
    &(ptr);
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {}
