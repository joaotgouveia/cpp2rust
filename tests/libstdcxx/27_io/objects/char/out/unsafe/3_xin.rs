extern crate libc;
use libc::*;
extern crate libcc2rs;
use libcc2rs::*;
use std::collections::BTreeMap;
use std::io::{Read, Seek, Write};
use std::os::fd::{AsFd, FromRawFd, IntoRawFd};
use std::rc::Rc;
pub unsafe fn test05_0() {
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*{
                    let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                    let __s: *mut ::std::fs::File = __r;
                    let _ =
                        (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello".as_ptr()).to_bytes());
                    __s
                });
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(&[(' ' as libc::c_char) as u8]);
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"world".as_ptr()).to_bytes());
            __s
        });
        let __s: *mut ::std::fs::File = __r;
        (libcc2rs::endl_unsafe as unsafe fn(*mut ::std::fs::File) -> *mut ::std::fs::File)(__s)
    };
    {
        let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
        let __s: *mut ::std::fs::File = __r;
        let _ =
            (*__s).write_all(::std::ffi::CStr::from_ptr(c"Enter your name: ".as_ptr()).to_bytes());
        __s
    };
    let mut s: Vec<libc::c_char> = vec![libc::c_char::default()];
    unimplemented!();
    {
        let __r: &mut ::std::fs::File = &mut (*{
            let __r: &mut ::std::fs::File = &mut (*{
                let __r: &mut ::std::fs::File = &mut (*libcc2rs::cout_unsafe());
                let __s: *mut ::std::fs::File = __r;
                let _ = (*__s).write_all(::std::ffi::CStr::from_ptr(c"hello ".as_ptr()).to_bytes());
                __s
            });
            let __s: *mut ::std::fs::File = __r;
            let mut __bytes = Vec::new();
            for &__c in &s[..s.len().saturating_sub(1)] {
                __c.push_stream_bytes(&mut __bytes);
            }
            let _ = ::std::io::Write::write_all(&mut *__s, &__bytes);
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
    (unsafe { test05_0() });
    return 0;
}
pub unsafe fn __cpp2rust_init_globals() {}
