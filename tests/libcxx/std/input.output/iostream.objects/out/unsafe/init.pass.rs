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
    pub cerr_mem_dump: *mut libc::c_char,
    pub cin_mem_dump: *mut libc::c_char,
    pub cout_mem_dump: *mut libc::c_char,
    pub clog_mem_dump: *mut libc::c_char,
}
impl Checker {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            cerr_mem_dump: Box::leak(
                (0..::std::mem::size_of::<::std::fs::File>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            cin_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            cout_mem_dump: Box::leak(
                (0..::std::mem::size_of::<::std::fs::File>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            clog_mem_dump: Box::leak(
                (0..::std::mem::size_of::<::std::fs::File>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
        };
        {
            if ::std::mem::size_of::<::std::fs::File>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*libcc2rs::cerr_unsafe()) as *mut ::std::fs::File)
                        as *mut libc::c_char) as *const ::libc::c_void),
                    (this.cerr_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<::std::fs::File>() as usize,
                )
            }
            (this.cerr_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<()>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ()) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.cin_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<()>() as usize,
                )
            }
            (this.cin_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<::std::fs::File>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*libcc2rs::cout_unsafe()) as *mut ::std::fs::File)
                        as *mut libc::c_char) as *const ::libc::c_void),
                    (this.cout_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<::std::fs::File>() as usize,
                )
            }
            (this.cout_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<::std::fs::File>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ::std::fs::File) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.clog_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<::std::fs::File>() as usize,
                )
            }
            (this.clog_mem_dump as *mut ::libc::c_void)
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.cerr_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.cin_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.cout_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.clog_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
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
    pub wcerr_mem_dump: *mut libc::c_char,
    pub wcin_mem_dump: *mut libc::c_char,
    pub wcout_mem_dump: *mut libc::c_char,
    pub wclog_mem_dump: *mut libc::c_char,
}
impl WideChecker {
    pub unsafe fn new() -> Self {
        let mut this = Self {
            wcerr_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            wcin_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            wcout_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
            wclog_mem_dump: Box::leak(
                (0..::std::mem::size_of::<()>())
                    .map(|_| (0 as libc::c_char))
                    .collect::<Box<[libc::c_char]>>(),
            )
            .as_mut_ptr(),
        };
        {
            if ::std::mem::size_of::<()>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ()) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.wcerr_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<()>() as usize,
                )
            }
            (this.wcerr_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<()>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ()) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.wcin_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<()>() as usize,
                )
            }
            (this.wcin_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<()>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ()) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.wcout_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<()>() as usize,
                )
            }
            (this.wcout_mem_dump as *mut ::libc::c_void)
        };
        {
            if ::std::mem::size_of::<()>() != 0 {
                ::std::ptr::copy_nonoverlapping(
                    (((&raw mut (*unimplemented!()) as *mut ()) as *mut libc::c_char)
                        as *const ::libc::c_void),
                    (this.wclog_mem_dump as *mut ::libc::c_void),
                    ::std::mem::size_of::<()>() as usize,
                )
            }
            (this.wclog_mem_dump as *mut ::libc::c_void)
        };
        this
    }
    pub unsafe fn destructor(&mut self) {
        {
            let __p = self.wcerr_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.wcin_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.wcout_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
                )))
            }
        };
        {
            let __p = self.wclog_mem_dump;
            if !__p.is_null() {
                ::std::mem::drop(Box::from_raw(::std::slice::from_raw_parts_mut(
                    __p,
                    libcc2rs::malloc_usable_size(__p as *mut ::libc::c_void)
                        / ::std::mem::size_of::<libc::c_char>(),
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
unsafe fn main_0(mut _a0: i32, mut _a1: *mut *mut libc::c_char) -> i32 {
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut check_0)).cerr_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*libcc2rs::cerr_unsafe()) as *mut ::std::fs::File)
                    as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut check_0)).cin_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ()) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut check_0)).cout_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*libcc2rs::cout_unsafe()) as *mut ::std::fs::File)
                    as *const libc::c_char) as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut check_0)).clog_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ::std::fs::File) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<::std::fs::File>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut wide_check_1)).wcerr_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ()) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut wide_check_1)).wcin_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ()) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut wide_check_1)).wcout_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ()) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    assert!(
        (({
            let sa = core::slice::from_raw_parts(
                ((*std::cell::LazyCell::force_mut(&mut *&raw mut wide_check_1)).wclog_mem_dump
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let sb = core::slice::from_raw_parts(
                (((&raw mut (*unimplemented!()) as *mut ()) as *const libc::c_char)
                    as *const ::libc::c_void) as *const u8,
                ::std::mem::size_of::<()>() as usize,
            );
            let mut diff = 0_i32;
            for (x, y) in sa.iter().zip(sb.iter()) {
                if x != y {
                    diff = (*x as i32) - (*y as i32);
                    break;
                }
            }
            diff
        }) == (0))
    );
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {
    std::cell::LazyCell::force(&*&raw const check_0);
    std::cell::LazyCell::force(&*&raw const wide_check_1);
}
