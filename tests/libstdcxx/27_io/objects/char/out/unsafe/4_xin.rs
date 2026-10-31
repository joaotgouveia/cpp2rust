extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test10_0() {
    {
        let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
        let __s: *mut ::std::fs::File = __r;
        let _ =
            (*__s).write_all(::std::ffi::CStr::from_ptr(c"Press ENTER once\n".as_ptr()).to_bytes());
        __s
    };
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s)
                    .write_all(::std::ffi::CStr::from_ptr(c"_M_gcount: ".as_ptr()).to_bytes());
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            libcc2rs::write_int(&mut *__s, (unimplemented!() as i64));
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
}
pub fn main() {
    unsafe {
        __cpp2rust_init_globals();
        std::process::exit(main_0() as i32);
    }
}
unsafe fn main_0() -> i32 {
    (unsafe { test10_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
