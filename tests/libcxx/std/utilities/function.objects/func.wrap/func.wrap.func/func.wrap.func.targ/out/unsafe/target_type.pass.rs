extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(VaArg, FnPtrArg, CopyAssignUnsafe, DestructorUnsafe)]
pub struct A {
    data_: [(); 10],
}
impl A {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            data_: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        let mut i: () = 0_();
        'loop_: while ((i) < (10_())) {
            this.data_[(i) as usize] = i;
            i.prefix_inc();
        }
        this
    }
    pub unsafe fn copy_from(_a0: *const A) -> Self {
        let mut this = Self {
            data_: [unimplemented!(); 10],
        };
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_inc();
        this
    }
    pub unsafe fn destructor(&mut self) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut count_0)).prefix_dec();
    }
    pub unsafe fn operator_call(&self, mut i: ()) -> () {
        let mut j: () = 0_();
        'loop_: while ((j) < (10_())) {
            i += self.data_[(j) as usize];
            j.prefix_inc();
        }
        return i;
    }
    pub unsafe fn foo(&self, mut _a0: ()) -> () {
        return 1_();
    }
    pub unsafe fn copy_assign(&mut self, _a0: *const A) -> *mut A {
        unimplemented!();
        return &mut (*(self as *mut A));
    }
}
impl Clone for A {
    fn clone(&self) -> Self {
        unsafe { A::copy_from(self as *const A) }
    }
}
impl Default for A {
    fn default() -> Self {
        unsafe { A::new() }
    }
}
pub static mut count_0: std::cell::LazyCell<()> = std::cell::LazyCell::new(|| unsafe { 0_() });
pub unsafe fn g_1(mut _a0: ()) -> () {
    return 0_();
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
    {
        let mut f: () = std_function_int__int__::new_1({ A::new() });
        unimplemented!();
    }
    {
        let mut f: () = unimplemented!();
        unimplemented!();
    }
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const count_0);
}
