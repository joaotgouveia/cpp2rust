extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub static mut allocated__0: std::cell::LazyCell<u64> =
    std::cell::LazyCell::new(|| unsafe { 0_u64 });
pub static mut deallocated__1: std::cell::LazyCell<u64> =
    std::cell::LazyCell::new(|| unsafe { 0_u64 });
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_char__unsigned_int__rebind_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_alloc_char__unsigned_int_ {}
impl test_alloc_char__unsigned_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)).wrapping_add((n as u64));
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1))
                .wrapping_add((s as u64));
        unimplemented!();
    }
}
impl Default for test_alloc_char__unsigned_int_ {
    fn default() -> Self {
        unsafe { test_alloc_char__unsigned_int_::new() }
    }
}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg, Default)]
pub struct test_alloc_char__unsigned_long__rebind_char_ {}
#[repr(C)]
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_alloc_char__unsigned_long_ {}
impl test_alloc_char__unsigned_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(&mut self, mut n: (), mut _a1: Option<()>) -> *mut () {
        let mut _a1: () = _a1.unwrap_or_else(|| unsafe { unimplemented!() });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)) =
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)) as ()).wrapping_add(n))
                as u64;
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut (), mut s: ()) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)) =
            (((*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)) as ())
                .wrapping_add(s)) as u64;
        unimplemented!();
    }
}
impl Default for test_alloc_char__unsigned_long_ {
    fn default() -> Self {
        unsafe { test_alloc_char__unsigned_long_::new() }
    }
}
pub unsafe fn test_2() {
    let mut i: () = 1_();
    'loop_: while ((i) < (1000_())) {
        {
            let mut s: () = unimplemented!();
            &(s);
        }
        unimplemented!();
        i.prefix_inc();
    }
}
pub unsafe fn test_3() {
    let mut i: () = 1_();
    'loop_: while ((i) < (1000_())) {
        {
            let mut s: () = unimplemented!();
            &(s);
        }
        unimplemented!();
        i.prefix_inc();
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
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_3() });
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const allocated__0);
    std::cell::LazyCell::force(&*&raw const deallocated__1);
}
