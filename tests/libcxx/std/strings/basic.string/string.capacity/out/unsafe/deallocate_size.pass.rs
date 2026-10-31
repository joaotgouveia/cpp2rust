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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_alloc_char__unsigned_int_ {}
impl test_alloc_char__unsigned_int_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u32,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)).wrapping_add((n as u64));
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut s: u32) {
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
#[derive(Copy, Clone, VaArg, FnPtrArg)]
pub struct test_alloc_char__unsigned_long_ {}
impl test_alloc_char__unsigned_long_ {
    pub unsafe fn new() -> Self {
        let mut this = Self {};
        this
    }
    pub unsafe fn allocate(
        &mut self,
        mut n: u64,
        mut _a1: Option<*const ::libc::c_void>,
    ) -> *mut libc::c_char {
        let mut _a1: *const ::libc::c_void = _a1.unwrap_or_else(|| unsafe { std::ptr::null() });
        (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0)).wrapping_add(n);
        return unimplemented!();
    }
    pub unsafe fn deallocate(&mut self, mut p: *mut libc::c_char, mut s: u64) {
        (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)) =
            (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)).wrapping_add(s);
        unimplemented!();
    }
}
impl Default for test_alloc_char__unsigned_long_ {
    fn default() -> Self {
        unsafe { test_alloc_char__unsigned_long_::new() }
    }
}
pub unsafe fn test_2() {
    let mut i: u32 = 1_u32;
    'loop_: while ((i) < (1000_u32)) {
        {
            let mut s: Vec<libc::c_char> = vec![('t' as libc::c_char); (i as usize) as usize]
                .iter()
                .cloned()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect();
            &(s);
        }
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0))
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)))
        );
        i.prefix_inc();
    }
}
pub unsafe fn test_3() {
    let mut i: u32 = 1_u32;
    'loop_: while ((i) < (1000_u32)) {
        {
            let mut s: Vec<libc::c_char> = vec![('t' as libc::c_char); (i as usize) as usize]
                .iter()
                .cloned()
                .chain(std::iter::once((0 as libc::c_char)))
                .collect();
            &(s);
        }
        assert!(
            ((*std::cell::LazyCell::force_mut(&mut *&raw mut allocated__0))
                == (*std::cell::LazyCell::force_mut(&mut *&raw mut deallocated__1)))
        );
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    (unsafe { test_2() });
    (unsafe { test_3() });
    (unsafe { test_3() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const allocated__0);
    std::cell::LazyCell::force(&*&raw const deallocated__1);
}
