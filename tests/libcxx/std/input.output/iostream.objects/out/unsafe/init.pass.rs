extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct Checker {
    pub cerr_mem_dump: *mut (),
    pub cin_mem_dump: *mut (),
    pub cout_mem_dump: *mut (),
    pub clog_mem_dump: *mut (),
}
impl Checker {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            cerr_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            cin_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            cout_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            clog_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.cerr_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.cin_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.cout_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.clog_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
impl Default for Checker {
    fn default() -> Self {
        unsafe { Checker::new() }
    }
}
pub static mut check_0: std::cell::LazyCell<Checker> =
    std::cell::LazyCell::new(|| unsafe { Checker::new() });
#[repr(C)]
#[derive(Clone, VaArg, FnPtrArg, DestructorUnsafe)]
pub struct WideChecker {
    pub wcerr_mem_dump: *mut (),
    pub wcin_mem_dump: *mut (),
    pub wcout_mem_dump: *mut (),
    pub wclog_mem_dump: *mut (),
}
impl WideChecker {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            wcerr_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            wcin_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            wcout_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
            wclog_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| unimplemented!())
                    .collect::<Box<[()]>>(),
            )
            .as_mut_ptr(),
        };
        unimplemented!();
        unimplemented!();
        unimplemented!();
        unimplemented!();
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.wcerr_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.wcin_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.wcout_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
        {
            let __p = self.wclog_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<()>(),
                )))
            }
        };
    }
}
impl Default for WideChecker {
    fn default() -> Self {
        unsafe { WideChecker::new() }
    }
}
pub static mut wide_check_1: std::cell::LazyCell<WideChecker> =
    std::cell::LazyCell::new(|| unsafe { WideChecker::new() });

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
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    unimplemented!();
    return 0_();
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const check_0);
    std::cell::LazyCell::force(&*&raw const wide_check_1);
}
